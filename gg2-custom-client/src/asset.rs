use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::prelude::*;

pub mod error;
pub mod identifier;
pub mod pack;

#[derive(Debug, Default)]
pub struct AssetServer {
    /// All loaded packs in ascending priority
    loaded_packs: Vec<AssetPack>,
    textures: HashMap<AssetId, ImageBufferU8>,
}

impl AssetServer {
    async fn read_texture(raw: &[u8]) -> error::Result<ImageBufferU8> {
        let image = image::load_from_memory_with_format(raw, image::ImageFormat::Png)?;

        Ok(image.to_rgba8())
    }

    async fn load_asset(path: &Path) -> error::Result<Vec<u8>> {
        Ok(tokio::fs::read(path).await?)
    }

    async fn load_texture(base: PathBuf, id: AssetId) -> error::Result<(AssetId, ImageBufferU8)> {
        let path = id.as_path(base, AssetType::Texture);
        trace!("Loading texture {} from: {}", id, path.display());

        let image_raw = Self::load_asset(&path).await?;
        Ok((id, Self::read_texture(&image_raw).await?))
    }

    // TODO: Replace with signal to render thread
    pub fn is_textures_empty(&self) -> bool {
        self.textures.is_empty()
    }

    pub fn take_textures(&mut self) -> Vec<(AssetId, ImageBufferU8)> {
        let mut old_textures = HashMap::new();
        std::mem::swap(&mut self.textures, &mut old_textures);

        old_textures.into_iter().collect()
    }

    pub async fn load_packs(&mut self, packs: &[PathBuf]) -> error::Result<()> {
        self.loaded_packs.reserve(packs.len());

        for path in packs {
            let asset_pack = AssetPack::from_path(path).await?;
            self.loaded_packs.push(asset_pack);
        }

        let mut asset_map = HashMap::new();

        for pack in &self.loaded_packs {
            pack.scan_files(&mut asset_map)?;
        }

        let mut set = tokio::task::JoinSet::new();

        for (asset_id, pack_path) in
            asset_map
                .into_iter()
                .filter_map(|((asset_type, asset_id), pack_path)| match asset_type {
                    AssetType::Texture => Some((asset_id, pack_path)),
                    AssetType::Map => None,
                })
        {
            set.spawn(Self::load_texture(pack_path.as_ref().clone(), asset_id));
        }

        for image in set.join_all().await {
            match image {
                Ok((image_id, image_buffer)) => {
                    self.textures.insert(image_id, image_buffer);
                }
                Err(error) => error!("Asset Error: {}", error),
            }
        }

        Ok(())
    }
}
