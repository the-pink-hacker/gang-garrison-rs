use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
    str::FromStr,
};

use string_path::SPathBuf;

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
    pub fn as_path(&self) -> &Path {
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
pub struct AssetId {
    namespace: String,
    path: SPathBuf,
}

impl AssetId {
    #[must_use]
    #[inline]
    pub fn new(namespace: String, path: impl Into<SPathBuf>) -> Self {
        Self {
            namespace,
            path: path.into(),
        }
    }

    #[must_use]
    pub fn gg2(path: impl Into<SPathBuf>) -> Self {
        Self::_gg2(path.into())
    }

    fn _gg2(path: SPathBuf) -> Self {
        Self::new(DEFAULT_NAMESPACE.to_string(), path)
    }

    pub fn as_path(&self, mut base: PathBuf, asset_type: AssetType) -> PathBuf {
        base.push(&self.namespace);
        base.push(asset_type.as_path());
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

        let asset_type = match path_parts.next() {
            Some("textures") => Ok(AssetType::Texture),
            Some("maps") => Ok(AssetType::Map),
            Some("sprites") => Ok(AssetType::Sprite),
            _ => Err(AssetError::InvalidAssetPath(path.to_path_buf())),
        }?;

        if let Some(full_file_name) = path_parts.as_mut_slice().last_mut()
            && let Some((file_name, _extension)) = full_file_name.split_once(".")
        {
            *full_file_name = file_name;
        }

        let path = SPathBuf::from_iter(path_parts);

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

        let path = SPathBuf::from(path_raw);

        Ok(Self::new(namespace.to_string(), path))
    }
}
