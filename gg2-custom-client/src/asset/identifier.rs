use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

const DEFAULT_NAMESPACE: &str = "gg2";

#[derive(Debug)]
pub enum AssetType {
    Texture,
    Map,
}

impl AssetType {
    pub fn as_path(&self) -> &Path {
        match self {
            Self::Texture => Path::new("textures"),
            Self::Map => Path::new("maps"),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
#[repr(transparent)]
pub struct AssetPath(Vec<String>);

impl<P: ToString> FromIterator<P> for AssetPath {
    #[inline]
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Self(iter.into_iter().map(|x| x.to_string()).collect())
    }
}

impl IntoIterator for AssetPath {
    type IntoIter = std::vec::IntoIter<String>;
    type Item = String;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a AssetPath {
    type IntoIter = std::slice::Iter<'a, String>;
    type Item = &'a String;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut AssetPath {
    type IntoIter = std::slice::IterMut<'a, String>;
    type Item = &'a mut String;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Display for AssetPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.join("/"))
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct AssetId {
    namespace: String,
    path: AssetPath,
}

impl AssetId {
    pub fn new(namespace: String, path: AssetPath) -> Self {
        Self { namespace, path }
    }

    pub fn gg2(path: AssetPath) -> Self {
        Self::new(DEFAULT_NAMESPACE.to_string(), path)
    }

    pub fn is_default_namespace(&self) -> bool {
        self.namespace == DEFAULT_NAMESPACE
    }

    pub fn as_path(&self, mut base: PathBuf, asset_type: AssetType) -> PathBuf {
        base.push(&self.namespace);
        base.push(asset_type.as_path());
        base.extend(&self.path);

        base
    }
}

impl Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_default_namespace() {
            write!(f, "{}:{}", self.namespace, self.path)
        } else {
            Display::fmt(&self.path, f)
        }
    }
}
