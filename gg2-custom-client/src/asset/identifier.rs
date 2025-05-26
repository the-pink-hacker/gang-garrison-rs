use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::prelude::*;

mod serde;

const DEFAULT_NAMESPACE: &str = "gg2";

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum AssetType {
    Texture,
    Map,
    Sprite,
}

impl AssetType {
    pub fn into_path(&self) -> &Path {
        match self {
            Self::Texture => Path::new("textures"),
            Self::Map => Path::new("maps"),
            Self::Sprite => Path::new("sprites"),
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Self::Texture => "png",
            Self::Map => "png",
            Self::Sprite => "toml",
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct AssetPath(Vec<String>);

impl AssetPath {
    pub fn strip_extension(&mut self) {
        if let Some(last) = self.0.iter_mut().next_back()
            && let Some(extension_index) = last.find('.')
        {
            let _extension = last.split_off(extension_index);
        }
    }

    /// The last part of the asset path
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.0.last().map(String::as_str)
    }

    pub fn pop(&mut self) -> bool {
        self.0.pop().is_some()
    }
}

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

impl Debug for AssetPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0.join("/"))
    }
}

impl From<String> for AssetPath {
    fn from(value: String) -> Self {
        Self::from_iter(value.split('/'))
    }
}

impl From<&str> for AssetPath {
    fn from(value: &str) -> Self {
        Self::from_iter(value.split('/'))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetId {
    namespace: String,
    path: AssetPath,
}

impl AssetId {
    #[must_use]
    #[inline]
    pub fn new(namespace: String, path: impl Into<AssetPath>) -> Self {
        Self {
            namespace,
            path: path.into(),
        }
    }

    #[must_use]
    pub fn gg2(path: impl Into<AssetPath>) -> Self {
        Self::_gg2(path.into())
    }

    fn _gg2(path: AssetPath) -> Self {
        Self::new(DEFAULT_NAMESPACE.to_string(), path)
    }

    pub fn into_path(&self, mut base: PathBuf, asset_type: AssetType) -> PathBuf {
        base.push(&self.namespace);
        base.push(asset_type.into_path());
        base.extend(&self.path);
        base.set_extension(asset_type.extension());

        base
    }

    pub fn from_path(path: &Path) -> Result<(AssetType, Self), AssetError> {
        let mut path_parts = path
            .iter()
            .map(std::ffi::OsStr::to_str)
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| AssetError::InvalidAssetPath(path.to_path_buf()))?
            .into_iter();

        let namespace = path_parts
            .next()
            .ok_or_else(|| AssetError::InvalidAssetPath(path.to_path_buf()))?
            .to_string();

        let (asset_type, mut path) = match path_parts.next() {
            Some("textures") => Ok((AssetType::Texture, AssetPath::from_iter(path_parts))),
            Some("maps") => Ok((AssetType::Map, AssetPath::from_iter(path_parts))),
            Some("sprites") => Ok((AssetType::Sprite, AssetPath::from_iter(path_parts))),
            _ => Err(AssetError::InvalidAssetPath(path.to_path_buf())),
        }?;

        path.strip_extension();

        Ok((asset_type, Self { namespace, path }))
    }

    #[inline]
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name()
    }

    #[inline]
    pub fn pop(&mut self) -> bool {
        self.path.pop()
    }
}

impl Debug for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}:{}\"", self.namespace, self.path)
    }
}

impl Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for AssetId {
    type Err = AssetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (namespace, path_raw) = s
            .split_once(':')
            .ok_or_else(|| AssetError::IdNamespace(s.to_string()))?;

        let path = AssetPath::from_iter(path_raw.split('/'));

        Ok(Self::new(namespace.to_string(), path))
    }
}
