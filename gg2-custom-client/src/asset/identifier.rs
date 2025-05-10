use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use crate::prelude::*;

const DEFAULT_NAMESPACE: &str = "gg2";

#[derive(Debug, Hash, PartialEq, Eq)]
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

#[derive(Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct AssetPath(Vec<String>);

impl AssetPath {
    pub fn strip_extension(&mut self) {
        if let Some(last) = self.0.iter_mut().next_back() {
            if let Some(extension_index) = last.find('.') {
                let _extension = last.split_off(extension_index);
            }
        }
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

#[derive(Hash, PartialEq, Eq)]
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

    pub fn from_path(path: &Path) -> std::result::Result<(AssetType, Self), AssetError> {
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
            _ => Err(AssetError::InvalidAssetPath(path.to_path_buf())),
        }?;

        path.strip_extension();

        Ok((asset_type, Self { namespace, path }))
    }
}

impl Debug for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}:{}\"", self.namespace, self.path)
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
