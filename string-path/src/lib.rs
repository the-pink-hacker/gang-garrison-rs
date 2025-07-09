use std::{error::Error, ffi::OsString, iter::FusedIterator, str::FromStr};

#[cfg(feature = "serde")]
mod serde;

pub const MAIN_SEPARATOR: char = '/';
pub const MAIN_SEPARATOR_STR: &str = "/";
const MAIN_SEPARATOR_BYTE: u8 = b'/';

#[inline]
fn is_sep_byte(value: u8) -> bool {
    value == MAIN_SEPARATOR_BYTE
}

fn iter_after<'a, 'b, I, J>(mut iter: I, mut prefix: J) -> Option<I>
where
    I: Iterator<Item = Component<'a>> + Clone,
    J: Iterator<Item = Component<'b>>,
{
    loop {
        let mut iter_next = iter.clone();
        match (iter_next.next(), prefix.next()) {
            (Some(ref x), Some(ref y)) if x == y => (),
            (Some(_), Some(_)) => return None,
            (Some(_), None) => return Some(iter),
            (None, None) => return Some(iter),
            (None, Some(_)) => return None,
        }
        iter = iter_next;
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
enum State {
    StartDir = 0,
    Body = 1,
    Done = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Component<'a> {
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'a str),
}

impl<'a> Component<'a> {
    pub fn as_str(self) -> &'a str {
        match self {
            Self::RootDir => MAIN_SEPARATOR_STR,
            Self::CurDir => ".",
            Self::ParentDir => "..",
            Self::Normal(path) => path,
        }
    }
}

impl AsRef<str> for Component<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<SPath> for Component<'_> {
    #[inline]
    fn as_ref(&self) -> &SPath {
        self.as_str().as_ref()
    }
}

#[derive(Clone)]
pub struct Components<'a> {
    path: &'a [u8],
    has_physical_root: bool,
    front: State,
    back: State,
}

#[derive(Clone)]
pub struct Iter<'a> {
    inner: Components<'a>,
}

impl std::fmt::Debug for Components<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DebugHelper<'a>(&'a SPath);

        impl std::fmt::Debug for DebugHelper<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_list().entries(self.0.components()).finish()
            }
        }

        f.debug_tuple("Components")
            .field(&DebugHelper(self.as_spath()))
            .finish()
    }
}

impl<'a> Components<'a> {
    #[inline]
    fn len_before_body(&self) -> usize {
        let root = if self.front <= State::StartDir && self.has_physical_root {
            1
        } else {
            0
        };
        let cur_dir = if self.front <= State::StartDir && self.include_cur_dir() {
            1
        } else {
            0
        };
        root + cur_dir
    }

    #[inline]
    fn finished(&self) -> bool {
        self.front == State::Done || self.back == State::Done || self.front > self.back
    }

    #[must_use]
    pub fn as_spath(&self) -> &'a SPath {
        let mut comps = self.clone();
        if comps.front == State::Body {
            comps.trim_left();
        }
        if comps.back == State::Body {
            comps.trim_right();
        }

        unsafe { SPath::from_u8_slice(comps.path) }
    }

    fn include_cur_dir(&self) -> bool {
        if self.has_physical_root {
            return false;
        }
        let mut iter = self.path.iter();
        match (iter.next(), iter.next()) {
            (Some(&b'.'), None) => true,
            (Some(&b'.'), Some(&b)) => is_sep_byte(b),
            _ => false,
        }
    }

    unsafe fn parse_single_component<'b>(&self, comp: &'b [u8]) -> Option<Component<'b>> {
        match comp {
            b"." => None,
            b".." => Some(Component::ParentDir),
            b"" => None,
            _ => Some(Component::Normal(unsafe { str::from_utf8_unchecked(comp) })),
        }
    }

    fn parse_next_component(&self) -> (usize, Option<Component<'a>>) {
        debug_assert!(self.front == State::Body);
        let (extra, comp) = match self.path.iter().cloned().position(is_sep_byte) {
            None => (0, self.path),
            Some(i) => (1, &self.path[..i]),
        };

        // SAFETY: `comp` is a valid substring, since it is split on a separator.
        (comp.len() + extra, unsafe {
            self.parse_single_component(comp)
        })
    }

    fn parse_next_component_back(&self) -> (usize, Option<Component<'a>>) {
        debug_assert!(self.back == State::Body);
        let start = self.len_before_body();
        let (extra, comp) = match self.path[start..].iter().cloned().rposition(is_sep_byte) {
            None => (0, &self.path[start..]),
            Some(i) => (1, &self.path[start + i + 1..]),
        };
        (comp.len() + extra, unsafe {
            self.parse_single_component(comp)
        })
    }

    fn trim_left(&mut self) {
        while !self.path.is_empty() {
            let (size, comp) = self.parse_next_component();
            if comp.is_some() {
                return;
            } else {
                self.path = &self.path[size..];
            }
        }
    }

    fn trim_right(&mut self) {
        while self.path.len() > self.len_before_body() {
            let (size, comp) = self.parse_next_component_back();
            if comp.is_some() {
                return;
            } else {
                self.path = &self.path[..self.path.len() - size]
            }
        }
    }
}

impl AsRef<SPath> for Components<'_> {
    #[inline]
    fn as_ref(&self) -> &SPath {
        self.as_spath()
    }
}

impl AsRef<str> for Components<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_spath().as_str()
    }
}

impl std::fmt::Debug for Iter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DebugHelper<'a>(&'a SPath);

        impl std::fmt::Debug for DebugHelper<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        f.debug_tuple("Iter")
            .field(&DebugHelper(self.as_spath()))
            .finish()
    }
}

impl<'a> Iter<'a> {
    #[must_use]
    #[inline]
    pub fn as_spath(&self) -> &'a SPath {
        self.inner.as_spath()
    }
}

impl AsRef<SPath> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &SPath {
        self.as_spath()
    }
}

impl AsRef<str> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_spath().as_str()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Component::as_str)
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(Component::as_str)
    }
}

impl FusedIterator for Iter<'_> {}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished() {
            match self.front {
                State::StartDir => {
                    self.front = State::Body;
                    if self.has_physical_root {
                        debug_assert!(!self.path.is_empty());
                        self.path = &self.path[1..];
                        return Some(Component::RootDir);
                    } else if self.include_cur_dir() {
                        debug_assert!(!self.path.is_empty());
                        self.path = &self.path[1..];
                        return Some(Component::CurDir);
                    }
                }
                State::Body if !self.path.is_empty() => {
                    let (size, comp) = self.parse_next_component();
                    self.path = &self.path[size..];
                    if comp.is_some() {
                        return comp;
                    }
                }
                State::Body => {
                    self.front = State::Done;
                }
                State::Done => unreachable!(),
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for Components<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while !self.finished() {
            match self.back {
                State::Body if self.path.len() > self.len_before_body() => {
                    let (size, comp) = self.parse_next_component_back();
                    self.path = &self.path[..self.path.len() - size];
                    if comp.is_some() {
                        return comp;
                    }
                }
                State::Body => self.back = State::StartDir,
                State::StartDir => {
                    self.back = State::Done;
                    if self.has_physical_root {
                        self.path = &self.path[..self.path.len() - 1];
                        return Some(Component::RootDir);
                    } else if self.include_cur_dir() {
                        self.path = &self.path[..self.path.len() - 1];
                        return Some(Component::CurDir);
                    }
                }
                State::Done => unreachable!(),
            }
        }
        None
    }
}

impl FusedIterator for Components<'_> {}

impl<'a> PartialEq for Components<'a> {
    #[inline]
    fn eq(&self, other: &Components<'a>) -> bool {
        let Components {
            path: _,
            front: _,
            back: _,
            has_physical_root: _,
        } = self;

        if self.path.len() == other.path.len()
            && self.front == other.front
            && self.back == State::Body
            && other.back == State::Body
            && self.path == other.path
        {
            return true;
        }

        Iterator::eq(self.clone().rev(), other.clone().rev())
    }
}

impl Eq for Components<'_> {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<'a> PartialOrd for Components<'a> {
    #[inline]
    fn partial_cmp(&self, other: &Components<'a>) -> Option<std::cmp::Ordering> {
        Some(compare_components(self.clone(), other.clone()))
    }
}

impl Ord for Components<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_components(self.clone(), other.clone())
    }
}

fn compare_components(mut left: Components<'_>, mut right: Components<'_>) -> std::cmp::Ordering {
    if left.front == right.front {
        let first_difference = match left.path.iter().zip(right.path).position(|(&a, &b)| a != b) {
            None if left.path.len() == right.path.len() => return std::cmp::Ordering::Equal,
            None => left.path.len().min(right.path.len()),
            Some(diff) => diff,
        };

        if let Some(previous_sep) = left.path[..first_difference]
            .iter()
            .rposition(|&b| is_sep_byte(b))
        {
            let mismatched_component_start = previous_sep + 1;
            left.path = &left.path[mismatched_component_start..];
            left.front = State::Body;
            right.path = &right.path[mismatched_component_start..];
            right.front = State::Body;
        }
    }

    Iterator::cmp(left, right)
}

#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Ancestors<'a> {
    next: Option<&'a SPath>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = &'a SPath;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(SPath::parent);
        next
    }
}

impl FusedIterator for Ancestors<'_> {}

pub struct SPathBuf {
    inner: String,
}

impl SPathBuf {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: String::with_capacity(capacity),
        }
    }

    #[must_use]
    #[inline]
    pub fn as_spath(&self) -> &SPath {
        self
    }

    #[inline]
    pub fn leak<'a>(self) -> &'a mut SPath {
        SPath::from_inner_mut(self.inner.leak())
    }

    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if `path` has a root but no prefix (e.g., `\windows`), it
    ///   replaces everything except for the prefix (if any) of `self`.
    /// * if `path` has a prefix but no root, it replaces `self`.
    /// * if `self` has a verbatim prefix (e.g. `\\?\C:\windows`)
    ///   and `path` is not empty, the new path is normalized: all references
    ///   to `.` and `..` are removed.
    ///
    /// Consider using [`Path::join`] if you need a new `PathBuf` instead of
    /// using this function on a cloned `PathBuf`.
    ///
    /// # Examples
    ///
    /// Pushing a relative path extends the existing path:
    ///
    /// ```
    /// use string_path::SPathBuf;
    ///
    /// let mut path = SPathBuf::from("/tmp");
    /// path.push("file.bk");
    /// assert_eq!(path, SPathBuf::from("/tmp/file.bk"));
    /// ```
    ///
    /// Pushing an absolute path replaces the existing path:
    ///
    /// ```
    /// use string_path::SPathBuf;
    ///
    /// let mut path = SPathBuf::from("/tmp");
    /// path.push("/etc");
    /// assert_eq!(path, SPathBuf::from("/etc"));
    /// ```
    pub fn push<P: AsRef<SPath>>(&mut self, path: P) {
        self._push(path.as_ref())
    }

    fn _push(&mut self, path: &SPath) {
        // in general, a separator is needed if the rightmost byte is not a separator
        let buf = self.inner.as_bytes();
        let need_sep = buf.last().map(|c| !is_sep_byte(*c)).unwrap_or(false);

        // absolute `path` replaces `self`
        if path.is_absolute() {
            self.inner.truncate(0);

        // `path` has a root but no prefix, e.g., `\windows` (Windows only)
        } else if need_sep {
            self.inner.push_str(MAIN_SEPARATOR_STR);
        }

        self.inner.push_str(path.as_str());
    }

    /// Truncates `self` to [`self.parent`].
    ///
    /// Returns `false` and does nothing if [`self.parent`] is [`None`].
    /// Otherwise, returns `true`.
    ///
    /// [`self.parent`]: Path::parent
    ///
    /// # Examples
    ///
    /// ```
    /// use string_path::{SPath, SPathBuf};
    ///
    /// let mut p = SPathBuf::from("/spirited/away.rs");
    ///
    /// p.pop();
    /// assert_eq!(SPath::new("/spirited"), p);
    /// p.pop();
    /// assert_eq!(SPath::new("/"), p);
    /// ```
    pub fn pop(&mut self) -> bool {
        match self.parent().map(|p| p.as_u8_slice().len()) {
            Some(len) => {
                self.inner.truncate(len);
                true
            }
            None => false,
        }
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity)
    }
}

impl Clone for SPathBuf {
    #[inline]
    fn clone(&self) -> Self {
        SPathBuf {
            inner: self.inner.clone(),
        }
    }

    /// Clones the contents of `source` into `self`.
    ///
    /// This method is preferred over simply assigning `source.clone()` to `self`,
    /// as it avoids reallocation if possible.
    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner)
    }
}

impl<T: ?Sized + AsRef<str>> From<&T> for SPathBuf {
    #[inline]
    fn from(value: &T) -> Self {
        Self::from(value.as_ref().to_string())
    }
}

impl From<String> for SPathBuf {
    #[inline]
    fn from(value: String) -> Self {
        Self { inner: value }
    }
}

impl From<SPathBuf> for String {
    #[inline]
    fn from(value: SPathBuf) -> Self {
        value.inner
    }
}

impl FromStr for SPathBuf {
    type Err = core::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<P: AsRef<SPath>> FromIterator<P> for SPathBuf {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        let mut buf = SPathBuf::new();
        buf.extend(iter);
        buf
    }
}

impl<P: AsRef<SPath>> Extend<P> for SPathBuf {
    fn extend<T: IntoIterator<Item = P>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |p| self.push(p.as_ref()));
    }
}

impl std::fmt::Debug for SPathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, f)
    }
}

impl std::fmt::Display for SPathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, f)
    }
}

impl std::ops::Deref for SPathBuf {
    type Target = SPath;

    #[inline]
    fn deref(&self) -> &Self::Target {
        SPath::new(&self.inner)
    }
}

impl std::ops::DerefMut for SPathBuf {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        SPath::from_inner_mut(&mut self.inner)
    }
}

impl std::borrow::Borrow<SPath> for SPathBuf {
    #[inline]
    fn borrow(&self) -> &SPath {
        self
    }
}

impl Default for SPathBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl std::borrow::ToOwned for SPath {
    type Owned = SPathBuf;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_spath_buf()
    }

    #[inline]
    fn clone_into(&self, target: &mut Self::Owned) {
        self.inner.clone_into(&mut target.inner)
    }
}

impl AsRef<str> for SPathBuf {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl PartialEq for SPathBuf {
    fn eq(&self, other: &SPathBuf) -> bool {
        self.components() == other.components()
    }
}

impl std::hash::Hash for SPathBuf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_spath().hash(state)
    }
}

impl Eq for SPathBuf {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for SPathBuf {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare_components(self.components(), other.components()))
    }
}

impl Ord for SPathBuf {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_components(self.components(), other.components())
    }
}

impl From<SPathBuf> for OsString {
    #[inline]
    fn from(value: SPathBuf) -> Self {
        value.inner.into()
    }
}

impl From<SPathBuf> for std::path::PathBuf {
    #[inline]
    fn from(value: SPathBuf) -> Self {
        value.inner.into()
    }
}

#[repr(transparent)]
pub struct SPath {
    inner: str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripPrefixError(());

impl std::fmt::Display for StripPrefixError {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for StripPrefixError {
    fn description(&self) -> &str {
        "prefix not found"
    }
}

impl SPath {
    #[inline]
    unsafe fn from_u8_slice(s: &[u8]) -> &Self {
        let path = unsafe { str::from_utf8_unchecked(s) };
        Self::new(path)
    }

    #[inline]
    fn as_u8_slice(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    #[inline]
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const str as *const Self) }
    }

    fn from_inner_mut(inner: &mut str) -> &mut Self {
        // SAFETY: Path is just a wrapper around OsStr,
        // therefore converting &mut OsStr to &mut Path is safe.
        unsafe { &mut *(inner as *mut str as *mut Self) }
    }

    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    #[must_use]
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        &mut self.inner
    }

    #[must_use]
    #[inline]
    pub fn to_spath_buf(&self) -> SPathBuf {
        SPathBuf::from(self.inner.to_string())
    }

    #[must_use]
    pub fn is_absolute(&self) -> bool {
        // TODO: Actually test if absolute
        self.components().has_physical_root
    }

    #[must_use]
    #[inline]
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub fn has_root(&self) -> bool {
        self.components().has_physical_root
    }

    #[must_use]
    pub fn parent(&self) -> Option<&SPath> {
        let mut comps = self.components();
        let comp = comps.next_back();
        comp.and_then(|p| match p {
            Component::Normal(_) | Component::CurDir | Component::ParentDir => {
                Some(comps.as_spath())
            }
            Component::RootDir => None,
        })
    }

    #[inline]
    pub fn ancestors(&self) -> Ancestors<'_> {
        Ancestors { next: Some(self) }
    }

    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.components().next_back().and_then(|p| match p {
            Component::Normal(p) => Some(p),
            _ => None,
        })
    }

    pub fn strip_prefix<P>(&self, base: P) -> Result<&SPath, StripPrefixError>
    where
        P: AsRef<SPath>,
    {
        self._strip_prefix(base.as_ref())
    }

    fn _strip_prefix(&self, base: &SPath) -> Result<&SPath, StripPrefixError> {
        iter_after(self.components(), base.components())
            .map(|c| c.as_spath())
            .ok_or(StripPrefixError(()))
    }

    /// Determines whether `base` is a prefix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_path::SPath;
    ///
    /// let path = SPath::new("/etc/passwd");
    ///
    /// assert!(path.starts_with("/etc"));
    /// assert!(path.starts_with("/etc/"));
    /// assert!(path.starts_with("/etc/passwd"));
    /// assert!(path.starts_with("/etc/passwd/")); // extra slash is okay
    /// assert!(path.starts_with("/etc/passwd///")); // multiple extra slashes are okay
    ///
    /// assert!(!path.starts_with("/e"));
    /// assert!(!path.starts_with("/etc/passwd.txt"));
    ///
    /// assert!(!SPath::new("/etc/foo.rs").starts_with("/etc/foo"));
    /// ```
    pub fn starts_with<P: AsRef<SPath>>(&self, base: P) -> bool {
        self._starts_with(base.as_ref())
    }

    fn _starts_with(&self, base: &SPath) -> bool {
        iter_after(self.components(), base.components()).is_some()
    }

    /// Determines whether `child` is a suffix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_path::SPath;
    ///
    /// let path = SPath::new("/etc/resolv.conf");
    ///
    /// assert!(path.ends_with("resolv.conf"));
    /// assert!(path.ends_with("etc/resolv.conf"));
    /// assert!(path.ends_with("/etc/resolv.conf"));
    ///
    /// assert!(!path.ends_with("/resolv.conf"));
    /// assert!(!path.ends_with("conf")); // use .extension() instead
    /// ```
    pub fn ends_with<P: AsRef<SPath>>(&self, base: P) -> bool {
        self._ends_with(base.as_ref())
    }

    fn _ends_with(&self, base: &SPath) -> bool {
        iter_after(self.components().rev(), base.components().rev()).is_some()
    }

    pub fn components(&self) -> Components<'_> {
        let bytes = self.as_u8_slice();
        Components {
            path: bytes,
            has_physical_root: !bytes.is_empty() && is_sep_byte(bytes[0]),
            front: State::StartDir,
            back: State::Body,
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.components(),
        }
    }
}

impl std::fmt::Debug for SPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl std::fmt::Display for SPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl PartialEq for SPath {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.components() == other.components()
    }
}

impl std::hash::Hash for SPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let bytes = self.as_u8_slice();

        let mut component_start = 0;
        // track some extra state to avoid prefix collisions.
        // ["foo", "bar"] and ["foobar"], will have the same payload bytes
        // but result in different chunk_bits
        let mut chunk_bits: usize = 0;

        for i in 0..bytes.len() {
            if is_sep_byte(bytes[i]) {
                if i > component_start {
                    let to_hash = &bytes[component_start..i];
                    chunk_bits = chunk_bits.wrapping_add(to_hash.len());
                    chunk_bits = chunk_bits.rotate_right(2);
                    state.write(to_hash);
                }

                // skip over separator and optionally a following CurDir item
                // since components() would normalize these away.
                component_start = i + 1;

                let tail = &bytes[component_start..];

                component_start += match tail {
                    [b'.'] => 1,
                    [b'.', sep, ..] if is_sep_byte(*sep) => 1,
                    _ => 0,
                };
            }
        }

        if component_start < bytes.len() {
            let to_hash = &bytes[component_start..];
            chunk_bits = chunk_bits.wrapping_add(to_hash.len());
            chunk_bits = chunk_bits.rotate_right(2);
            state.write(to_hash);
        }

        state.write_usize(chunk_bits);
    }
}

impl Eq for SPath {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for SPath {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare_components(self.components(), other.components()))
    }
}

impl Ord for SPath {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_components(self.components(), other.components())
    }
}

impl AsRef<SPath> for SPath {
    #[inline]
    fn as_ref(&self) -> &SPath {
        self
    }
}

impl AsRef<SPath> for str {
    #[inline]
    fn as_ref(&self) -> &SPath {
        SPath::new(self)
    }
}

impl AsRef<SPath> for String {
    #[inline]
    fn as_ref(&self) -> &SPath {
        SPath::new(self)
    }
}

impl AsRef<SPath> for SPathBuf {
    #[inline]
    fn as_ref(&self) -> &SPath {
        self
    }
}

impl<'a> IntoIterator for &'a SPathBuf {
    type Item = &'a str;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a SPath {
    type Item = &'a str;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

macro_rules! impl_cmp {
    (<$($life:lifetime),*> $lhs:ty, $rhs: ty) => {
        impl<$($life),*> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <SPath as PartialEq>::eq(self, other)
            }
        }

        impl<$($life),*> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <SPath as PartialEq>::eq(self, other)
            }
        }

        impl<$($life),*> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<std::cmp::Ordering> {
                <SPath as PartialOrd>::partial_cmp(self, other)
            }
        }

        impl<$($life),*> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<std::cmp::Ordering> {
                <SPath as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(<> SPathBuf, SPath);
impl_cmp!(<'a> SPathBuf, &'a SPath);
//impl_cmp!(<'a> Cow<'a, SPath>, SPath);
//impl_cmp!(<'a, 'b> Cow<'a, SPath>, &'b SPath);
//impl_cmp!(<'a> Cow<'a, SPath>, SPathBuf);

macro_rules! impl_cmp_os_str {
    (<$($life:lifetime),*> $lhs:ty, $rhs: ty) => {
        impl<$($life),*> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <SPath as PartialEq>::eq(self, other.as_ref())
            }
        }

        impl<$($life),*> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <SPath as PartialEq>::eq(self.as_ref(), other)
            }
        }

        impl<$($life),*> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<std::cmp::Ordering> {
                <SPath as PartialOrd>::partial_cmp(self, other.as_ref())
            }
        }

        impl<$($life),*> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<std::cmp::Ordering> {
                <SPath as PartialOrd>::partial_cmp(self.as_ref(), other)
            }
        }
    };
}

impl_cmp_os_str!(<> SPathBuf, str);
impl_cmp_os_str!(<'a> SPathBuf, &'a str);
//impl_cmp_os_str!(<'a> SPathBuf, Cow<'a, str>);
impl_cmp_os_str!(<> SPathBuf, String);
impl_cmp_os_str!(<> SPath, str);
impl_cmp_os_str!(<'a> SPath, &'a str);
//impl_cmp_os_str!(<'a> SPath, Cow<'a, str>);
impl_cmp_os_str!(<> SPath, String);
impl_cmp_os_str!(<'a> &'a SPath, str);
//impl_cmp_os_str!(<'a, 'b> &'a SPath, Cow<'b, str>);
impl_cmp_os_str!(<'a> &'a SPath, String);
//impl_cmp_os_str!(<'a> Cow<'a, SPath>, str);
//impl_cmp_os_str!(<'a, 'b> Cow<'a, SPath>, &'b str);
//impl_cmp_os_str!(<'a> Cow<'a, SPath>, String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spath_components_front_root() {
        let mut x = SPath::new("/this/is/./a/../test").components();
        assert_eq!(x.next(), Some(Component::RootDir));
        assert_eq!(x.next(), Some(Component::Normal("this")));
        assert_eq!(x.next(), Some(Component::Normal("is")));
        assert_eq!(x.next(), Some(Component::Normal("a")));
        assert_eq!(x.next(), Some(Component::ParentDir));
        assert_eq!(x.next(), Some(Component::Normal("test")));
    }

    #[test]
    fn spath_components_front_current() {
        let mut x = SPath::new("./this/is/./a/../test").components();
        assert_eq!(x.next(), Some(Component::CurDir));
        assert_eq!(x.next(), Some(Component::Normal("this")));
        assert_eq!(x.next(), Some(Component::Normal("is")));
        assert_eq!(x.next(), Some(Component::Normal("a")));
        assert_eq!(x.next(), Some(Component::ParentDir));
        assert_eq!(x.next(), Some(Component::Normal("test")));
    }

    #[test]
    fn spath_components_back_root() {
        let mut x = SPath::new("/../backwards/./am/i").components();
        assert_eq!(x.next_back(), Some(Component::Normal("i")));
        assert_eq!(x.next_back(), Some(Component::Normal("am")));
        assert_eq!(x.next_back(), Some(Component::Normal("backwards")));
        assert_eq!(x.next_back(), Some(Component::ParentDir));
        assert_eq!(x.next_back(), Some(Component::RootDir));
    }

    #[test]
    fn spath_components_back_current() {
        let mut x = SPath::new("./../backwards/./am/i").components();
        assert_eq!(x.next_back(), Some(Component::Normal("i")));
        assert_eq!(x.next_back(), Some(Component::Normal("am")));
        assert_eq!(x.next_back(), Some(Component::Normal("backwards")));
        assert_eq!(x.next_back(), Some(Component::ParentDir));
        assert_eq!(x.next_back(), Some(Component::CurDir));
    }

    #[test]
    fn spath_to_string() {
        let path = SPath::new("this/is/a/test");
        assert_eq!(path.to_string(), "this/is/a/test");
    }

    #[test]
    fn spath_buf_to_string() {
        let path = SPathBuf::from("this/is/a/test");
        assert_eq!(path.to_string(), "this/is/a/test");
    }
}
