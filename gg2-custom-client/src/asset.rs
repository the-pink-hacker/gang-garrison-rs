use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use sprite::SpriteContextAsset;

use crate::prelude::*;

pub mod error;
pub mod identifier;
pub mod pack;
pub mod sprite;

#[derive(Debug, Default)]
pub struct AssetServer {
    /// All loaded packs in ascending priority
    loaded_packs: Vec<AssetPack>,
    textures: HashMap<AssetId, ImageBufferU8>,
    sprites: HashMap<AssetId, SpriteContextAsset>,
}

impl AssetServer {
    async fn read_texture(raw: &[u8]) -> error::Result<ImageBufferU8> {
        let image = image::load_from_memory_with_format(raw, image::ImageFormat::Png)?;

        Ok(image.to_rgba8())
    }

    async fn load_asset(path: &Path) -> error::Result<Vec<u8>> {
        Ok(tokio::fs::read(path).await?)
    }

    async fn load_asset_string(path: &Path) -> error::Result<String> {
        Ok(tokio::fs::read_to_string(path).await?)
    }

    async fn load_texture(base: PathBuf, id: AssetId) -> error::Result<(AssetId, ImageBufferU8)> {
        let path = id.as_path(base, AssetType::Texture);
        trace!("Loading texture {} from: {}", id, path.display());

        let image_raw = Self::load_asset(&path).await?;
        Ok((id, Self::read_texture(&image_raw).await?))
    }

    async fn load_sprite(
        base: PathBuf,
        id: AssetId,
    ) -> error::Result<(AssetId, SpriteContextAsset)> {
        let path = id.as_path(base, AssetType::Sprite);
        trace!("Loading sprite {} from {}", id, path.display());

        let sprite_raw = Self::load_asset_string(&path).await?;
        let sprite = toml::from_str(&sprite_raw)?;

        Ok((id, sprite))
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

    pub fn get_sprite(&self, id: &AssetId) -> Result<&SpriteContextAsset, AssetError> {
        self.sprites
            .get(id)
            .ok_or_else(|| AssetError::AtlasLookup(id.clone()))
    }

    pub async fn load_packs(&mut self, packs: &[PathBuf]) -> error::Result<()> {
        self.loaded_packs.reserve(packs.len());

        for path in packs {
            let asset_pack = AssetPack::from_path(path).await?;

            if asset_pack.metadata.format != 0 {
                error!(
                    "Asset Pack \"{}\" has incompatible pack format: {} != 0",
                    asset_pack.metadata.name, asset_pack.metadata.format
                );
                continue;
            }

            debug!("Loading asset pack:\n{:#?}", asset_pack.metadata);

            self.loaded_packs.push(asset_pack);
        }

        let mut texture_map = HashMap::new();
        let mut sprite_map = HashMap::new();

        for pack in &self.loaded_packs {
            pack.scan_files(&mut texture_map, &mut sprite_map)?;
        }

        let mut texture_set = tokio::task::JoinSet::new();

        for (asset_id, pack_path) in texture_map {
            texture_set.spawn(Self::load_texture(pack_path.as_ref().clone(), asset_id));
        }

        let mut sprite_set = tokio::task::JoinSet::new();

        for (asset_id, pack_path) in sprite_map {
            sprite_set.spawn(Self::load_sprite(pack_path.as_ref().clone(), asset_id));
        }

        while let Some(Ok(sprite)) = sprite_set.join_next().await {
            match sprite {
                Ok((sprite_id, sprite_asset)) => {
                    self.sprites.insert(sprite_id, sprite_asset);
                }
                Err(error) => error!("Asset Error: {error}"),
            }
        }

        // Textures will likely take the longest to load
        while let Some(Ok(texture)) = texture_set.join_next().await {
            match texture {
                Ok((texture_id, texture_buffer)) => {
                    self.textures.insert(texture_id, texture_buffer);
                }
                Err(error) => error!("Asset Error: {error}"),
            }
        }

        Ok(())
    }
}
