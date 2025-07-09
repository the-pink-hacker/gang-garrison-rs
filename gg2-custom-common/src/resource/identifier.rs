use std::path::{Path, PathBuf};

use string_path::{SPath, SPathBuf};

use crate::prelude::*;

pub mod serde;

const DEFAULT_NAMESPACE: &str = "gg2";

pub trait ResourceType: AsRef<Path> + Copy {
    fn extension(&self) -> &str;

    fn from_folder(folder: &str) -> Option<Self>;

    fn get_id(path: &SPath) -> Result<(Self, ResourceId), ResourceError> {
        let mut path_parts = path.into_iter();

        let namespace = path_parts
            .next()
            .ok_or_else(|| ResourceError::InvalidResourcePath(path.to_spath_buf()))?
            .to_string();

        let asset_type = path_parts
            .next()
            .and_then(ResourceType::from_folder)
            .ok_or_else(|| ResourceError::InvalidResourcePath(path.to_spath_buf()))?;

        let mut path = SPathBuf::from_iter(path_parts);

        if let Some(full_file_name) = path.file_name()
            && let Some(extension_index) = full_file_name.find(".")
        {
            let truncated_file_name = full_file_name[..extension_index].to_string();
            path.pop();
            path.push(truncated_file_name);
        }

        Ok((asset_type, ResourceId { namespace, path }))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceId {
    namespace: String,
    path: SPathBuf,
}

impl ResourceId {
    #[must_use]
    #[inline]
    pub fn new(namespace: String, path: impl Into<SPathBuf>) -> Self {
        Self {
            namespace,
            path: path.into(),
        }
    }

    #[inline]
    #[must_use]
    pub fn gg2(path: impl Into<SPathBuf>) -> Self {
        Self::_gg2(path.into())
    }

    fn _gg2(path: SPathBuf) -> Self {
        Self::new(DEFAULT_NAMESPACE.to_string(), path)
    }

    pub fn as_path(&self, mut base: PathBuf, asset_type: impl ResourceType) -> PathBuf {
        base.push(&self.namespace);
        base.push(asset_type);
        base.extend(&self.path);
        base.set_extension(asset_type.extension());

        base
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

impl std::fmt::Debug for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}:{}\"", self.namespace, self.path)
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl std::str::FromStr for ResourceId {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (namespace, path_raw) = s
            .split_once(':')
            .ok_or_else(|| ResourceError::IdNamespace(s.to_string()))?;

        let path = SPathBuf::from(path_raw);

        Ok(Self::new(namespace.to_string(), path))
    }
}
