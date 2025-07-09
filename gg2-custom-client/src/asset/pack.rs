use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use string_path::SPathBuf;

use crate::prelude::*;

const PACK_TOML: &str = "pack.toml";

#[derive(Debug, Clone)]
pub struct AssetPack {
    pub metadata: AssetPackMetadata,
    /// Located at `/assets`
    asset_root: Arc<PathBuf>,
}

impl AssetPack {
    /// Constructs an `AssetPack` given the path that contains the `pack.toml`
    pub async fn from_path(pack_path: &Path) -> Result<Self, ResourceError> {
        let canon_path = pack_path
            .canonicalize()
            .map_err(|_| ResourceError::PackMeta(pack_path.to_path_buf()))?;
        let toml_path = canon_path.join(PACK_TOML);

        let toml_file = tokio::fs::read_to_string(&toml_path)
            .await
            .map_err(|_| ResourceError::PackMeta(toml_path.clone()))?;

        let metadata = toml::from_str::<AssetPackMetadataRoot>(&toml_file)
            .map_err(|error| ResourceError::PackMetaToml(toml_path, error))?
            .pack;

        Ok(Self {
            metadata,
            asset_root: canon_path.join("assets").into(),
        })
    }

    pub fn scan_files(
        &self,
        texture_map: &mut HashMap<ResourceId, Arc<PathBuf>>,
        sprite_map: &mut HashMap<ResourceId, Arc<PathBuf>>,
        map_map: &mut HashMap<ResourceId, Arc<PathBuf>>,
        atlas_map: &mut HashMap<ResourceId, Arc<PathBuf>>,
    ) -> Result<(), AssetError> {
        for asset_path in walkdir::WalkDir::new(&*self.asset_root)
            .follow_links(false)
            // Skip root directory
            .min_depth(1)
            .into_iter()
            .filter_map(std::result::Result::ok)
            // Skip folders
            .filter(|entry| {
                entry
                    .metadata()
                    .as_ref()
                    .map(std::fs::Metadata::is_file)
                    .unwrap_or_default()
            })
            .map(walkdir::DirEntry::into_path)
        {
            let relative_path = asset_path.strip_prefix(&*self.asset_root).map_err(|_| {
                ResourceError::StripPrefix((*self.asset_root).clone(), asset_path.clone())
            })?;
            let relative_spath = relative_path
                .to_str()
                .map(SPathBuf::from)
                .ok_or_else(|| ResourceError::InvalidStringPath(relative_path.to_path_buf()))?;
            let (asset_type, asset_id) = AssetType::get_id(&relative_spath)?;

            match asset_type {
                AssetType::Texture => {
                    texture_map.insert(asset_id, Arc::clone(&self.asset_root));
                }
                AssetType::Map => {
                    map_map.insert(asset_id, Arc::clone(&self.asset_root));
                }
                AssetType::Sprite => {
                    sprite_map.insert(asset_id, Arc::clone(&self.asset_root));
                }
                AssetType::Atlas => {
                    atlas_map.insert(asset_id, Arc::clone(&self.asset_root));
                }
            }
        }

        Ok(())
    }
}

/// A wrapper struct for serde that represents the `pack.toml` file
#[derive(Debug, Clone, Deserialize, Serialize)]
struct AssetPackMetadataRoot {
    pack: AssetPackMetadata,
}

/// All data stored in the pack's `pack.toml` file
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetPackMetadata {
    /// A human readable name for the pack
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: String,
    /// The pack's version
    pub version: semver::Version,
    /// Should be 0 until standard is formed
    pub format: u16,
}
