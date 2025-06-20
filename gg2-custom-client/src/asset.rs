use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::prelude::*;
use error::Result;

pub mod error;
pub mod identifier;
pub mod pack;
pub mod sprite;

#[derive(Debug, Default)]
pub struct AssetServer {
    /// All loaded packs in ascending priority
    loaded_packs: Vec<AssetPack>,
    textures: HashMap<ResourceId, ImageBufferRGBA8>,
    sprites: HashMap<ResourceId, SpriteContextAsset>,
    /// All scanned maps with the base pack path
    maps: HashMap<ResourceId, Arc<PathBuf>>,
}

impl AssetServer {
    async fn read_texture(buf: &[u8]) -> error::Result<ImageBufferRGBA8> {
        Ok(image::load_from_memory_with_format(buf, image::ImageFormat::Png)?.to_rgba8())
    }

    async fn load_asset(path: impl AsRef<Path>) -> Result<Vec<u8>> {
        Ok(tokio::fs::read(path).await?)
    }

    async fn load_asset_string(path: impl AsRef<Path>) -> Result<String> {
        Ok(tokio::fs::read_to_string(path).await?)
    }

    async fn load_texture(base: PathBuf, id: &ResourceId) -> error::Result<ImageBufferRGBA8> {
        let path = id.as_path(base, AssetType::Texture);
        trace!("Loading texture {} from: {}", id, path.display());

        let image_raw = Self::load_asset(&path).await?;
        Self::read_texture(&image_raw).await
    }

    async fn load_sprite(base: PathBuf, id: &ResourceId) -> Result<SpriteContextAsset> {
        let path = id.as_path(base, AssetType::Sprite);
        trace!("Loading sprite {} from {}", id, path.display());

        let sprite_raw = Self::load_asset_string(&path).await?;

        Ok(toml::from_str(&sprite_raw)?)
    }

    // TODO: Check map MD5
    pub async fn load_map(&self, id: &ResourceId) -> Result<(ImageBufferRGBA8, MapData)> {
        let base_path = self
            .maps
            .get(id)
            .ok_or_else(|| AssetError::Unloaded(AssetType::Map.to_string(), id.clone()))?
            .as_ref()
            .clone();
        let path = id.as_path(base_path, AssetType::Map);
        let map_buffer = Self::load_asset(path).await?;

        let map_data = MapData::load_from_memory(&map_buffer)?;
        let image =
            image::load_from_memory_with_format(&map_buffer, image::ImageFormat::Png)?.to_rgba8();

        Ok((image, map_data))
    }

    pub fn push_textures(&mut self, world: &ClientWorld) -> std::result::Result<(), ClientError> {
        let textures = std::mem::take(&mut self.textures).into_iter().collect();

        world
            .game_to_render_channel()
            .send(GameToRenderMessage::UpdateSpriteAtlas(textures))?;

        Ok(())
    }

    pub fn get_sprite(&self, id: &ResourceId) -> Result<&SpriteContextAsset> {
        self.sprites
            .get(id)
            .ok_or_else(|| AssetError::AtlasLookup(id.clone()))
    }

    pub async fn load_packs(&mut self, packs: &[PathBuf]) -> Result<()> {
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
            pack.scan_files(&mut texture_map, &mut sprite_map, &mut self.maps)?;
        }

        let mut texture_set = tokio::task::JoinSet::new();

        for (asset_id, pack_path) in texture_map {
            texture_set.spawn(async move {
                Self::load_texture(pack_path.as_ref().clone(), &asset_id)
                    .await
                    .map(|texture| (asset_id, texture))
            });
        }

        let mut sprite_set = tokio::task::JoinSet::new();

        for (asset_id, pack_path) in sprite_map {
            sprite_set.spawn(async move {
                Self::load_sprite(pack_path.as_ref().clone(), &asset_id)
                    .await
                    .map(|sprite| (asset_id, sprite))
            });
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
