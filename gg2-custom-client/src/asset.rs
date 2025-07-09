use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::prelude::*;
use tokio::task::JoinSet;

pub mod atlas;
pub mod error;
pub mod identifier;
pub mod pack;
pub mod sprite;

#[derive(Debug, Default)]
pub struct AssetServer {
    /// All loaded packs in ascending priority
    loaded_packs: Vec<AssetPack>,
    atlases: HashMap<ResourceId, AtlasDefinition>,
    textures: HashMap<ResourceId, ImageBufferRGBA8>,
    sprites: HashMap<ResourceId, SpriteContextAsset>,
    /// All scanned maps with the base pack path
    maps: HashMap<ResourceId, Arc<PathBuf>>,
}

impl AssetServer {
    async fn read_texture(buf: &[u8]) -> Result<ImageBufferRGBA8, AssetError> {
        Ok(image::load_from_memory_with_format(buf, image::ImageFormat::Png)?.to_rgba8())
    }

    async fn load_asset(path: impl AsRef<Path>) -> Result<Vec<u8>, AssetError> {
        Ok(tokio::fs::read(path).await?)
    }

    async fn load_asset_string(path: impl AsRef<Path>) -> Result<String, AssetError> {
        Ok(tokio::fs::read_to_string(path).await?)
    }

    // TODO: Check map MD5
    pub async fn load_map(
        &self,
        id: &ResourceId,
    ) -> Result<(ImageBufferRGBA8, MapData), AssetError> {
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

    pub fn push_textures(&mut self, world: &ClientWorld) -> Result<(), ClientError> {
        let textures = std::mem::take(&mut self.textures).into_iter().collect();

        world
            .game_to_render_channel()
            .send(GameToRenderMessage::UpdateSpriteAtlas(textures))?;

        Ok(())
    }

    pub fn get_sprite(&self, id: &ResourceId) -> Result<&SpriteContextAsset, AssetError> {
        self.sprites
            .get(id)
            .ok_or_else(|| AssetError::AtlasLookup(id.clone()))
    }

    pub async fn load_packs(&mut self, packs: &[PathBuf]) -> Result<(), AssetError> {
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
        let mut atlas_map = HashMap::new();

        for pack in &self.loaded_packs {
            pack.scan_files(
                &mut texture_map,
                &mut sprite_map,
                &mut self.maps,
                &mut atlas_map,
            )?;
        }

        let mut texture_set = Self::process_asset(texture_map).await;
        let mut sprite_set = Self::process_asset(sprite_map).await;
        let mut atlas_set = Self::process_asset(atlas_map).await;

        Self::store_assets(&mut sprite_set, &mut self.sprites).await;
        Self::store_assets(&mut atlas_set, &mut self.atlases).await;
        // Textures will likely take the longest to load
        Self::store_assets(&mut texture_set, &mut self.textures).await;

        Ok(())
    }

    async fn process_asset<T>(
        asset_map: HashMap<ResourceId, Arc<PathBuf>>,
    ) -> JoinSet<std::result::Result<(ResourceId, T), AssetError>>
    where
        T: Send + 'static,
        Self: AssetLoader<T>,
    {
        let mut join_set = JoinSet::new();

        asset_map.into_iter().for_each(|(asset_id, pack_path)| {
            join_set.spawn(async move {
                <Self as AssetLoader<T>>::load_asset(pack_path.to_path_buf(), &asset_id)
                    .await
                    .map(|asset| (asset_id, asset))
            });
        });

        join_set
    }

    async fn store_assets<T: 'static>(
        asset_set: &mut JoinSet<Result<(ResourceId, T), AssetError>>,
        asset_store: &mut HashMap<ResourceId, T>,
    ) {
        while let Some(Ok(join_result)) = asset_set.join_next().await {
            match join_result {
                Ok((asset_id, asset)) => {
                    asset_store.insert(asset_id, asset);
                }
                Err(error) => error!("Asset Error: {error}"),
            }
        }
    }
}

trait AssetLoader<T> {
    fn load_asset(
        pack_path: PathBuf,
        asset_id: &ResourceId,
    ) -> impl Future<Output = Result<T, AssetError>> + Send;
}

impl AssetLoader<ImageBufferRGBA8> for AssetServer {
    async fn load_asset(
        pack_path: PathBuf,
        asset_id: &ResourceId,
    ) -> Result<ImageBufferRGBA8, AssetError> {
        let path = asset_id.as_path(pack_path, AssetType::Texture);
        trace!("Loading texture {} from: {}", asset_id, path.display());

        let image_raw = Self::load_asset(&path).await?;
        Self::read_texture(&image_raw).await
    }
}

impl AssetLoader<SpriteContextAsset> for AssetServer {
    async fn load_asset(
        pack_path: PathBuf,
        asset_id: &ResourceId,
    ) -> Result<SpriteContextAsset, AssetError> {
        let path = asset_id.as_path(pack_path, AssetType::Sprite);
        trace!("Loading sprite {} from {}", asset_id, path.display());

        let sprite_raw = Self::load_asset_string(&path).await?;

        Ok(toml::from_str(&sprite_raw)?)
    }
}

impl AssetLoader<AtlasDefinition> for AssetServer {
    async fn load_asset(
        pack_path: PathBuf,
        asset_id: &ResourceId,
    ) -> Result<AtlasDefinition, AssetError> {
        let path = asset_id.as_path(pack_path, AssetType::Atlas);
        trace!("Loading atlas {} from {}", asset_id, path.display());

        let atlas_raw = Self::load_asset_string(&path).await?;

        Ok(toml::from_str(&atlas_raw)?)
    }
}
