use std::path::Path;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum AssetType {
    Texture,
    Map,
    Sprite,
    Atlas,
}

impl AsRef<Path> for AssetType {
    fn as_ref(&self) -> &Path {
        match self {
            Self::Texture => Path::new("textures"),
            Self::Map => Path::new("maps"),
            Self::Sprite => Path::new("sprites"),
            Self::Atlas => Path::new("atlases"),
        }
    }
}

impl ResourceType for AssetType {
    fn extension(&self) -> &str {
        match self {
            Self::Texture | Self::Map => "png",
            Self::Sprite | Self::Atlas => "toml",
        }
    }

    fn from_folder(folder: &str) -> Option<Self> {
        match folder {
            "textures" => Some(Self::Texture),
            "sprites" => Some(Self::Sprite),
            "maps" => Some(Self::Map),
            "atlases" => Some(Self::Atlas),
            _ => None,
        }
    }
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
