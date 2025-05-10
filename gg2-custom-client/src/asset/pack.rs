use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

const PACK_TOML: &str = "pack.toml";

#[derive(Debug, Clone)]
pub struct AssetPack {
    metadata: AssetPackMetadata,
    /// The path that contains `pack.toml`
    pack_path: PathBuf,
    asset_root: Arc<PathBuf>,
}

impl AssetPack {
    /// Constructs an `AssetPack` given the path that contains the `pack.toml`
    pub async fn from_path(pack_path: &Path) -> std::result::Result<Self, AssetError> {
        let canon_path = pack_path
            .canonicalize()
            .map_err(|_| AssetError::PackMeta(pack_path.to_path_buf()))?;
        let toml_path = canon_path.join(PACK_TOML);

        let toml_file = tokio::fs::read_to_string(&toml_path)
            .await
            .map_err(|_| AssetError::PackMeta(toml_path.clone()))?;

        let metadata = toml::from_str::<AssetPackMetadataRoot>(&toml_file)
            .map_err(|error| AssetError::PackMetaToml(toml_path, error))?
            .pack;

        Ok(Self {
            metadata,
            asset_root: canon_path.join("assets").into(),
            pack_path: canon_path,
        })
    }

    pub fn scan_files(
        &self,
        asset_map: &mut HashMap<(AssetType, AssetId), Arc<PathBuf>>,
    ) -> std::result::Result<(), AssetError> {
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
                    .map(|metadata| metadata.is_file())
                    .unwrap_or_default()
            })
            .map(walkdir::DirEntry::into_path)
        {
            let relative_path = asset_path.strip_prefix(&*self.asset_root).map_err(|_| {
                AssetError::StripPrefix((*self.asset_root).clone(), asset_path.clone())
            })?;
            let asset_path_parsed = AssetId::from_path(relative_path)?;
            asset_map.insert(asset_path_parsed, Arc::clone(&self.asset_root));
        }

        Ok(())
    }
}

/// A wrapper struct for serde that represents the `pack.toml` file
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct AssetPackMetadataRoot {
    pack: AssetPackMetadata,
}

/// All data stored in the pack's `pack.toml` file
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetPackMetadata {
    /// A human readable name for the pack
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    license: String,
    /// The pack's version
    version: String,
    /// Should be 0 until standard is formed
    format: u8,
}
