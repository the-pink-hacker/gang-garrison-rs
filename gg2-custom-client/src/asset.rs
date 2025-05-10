use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::prelude::*;

pub mod identifier;
pub mod pack;

pub mod error {
    use std::path::PathBuf;

    pub type Result<T> = std::result::Result<T, AssetError>;

    #[derive(Debug, thiserror::Error)]
    pub enum AssetError {
        #[error("Failed to load asset: {0}")]
        Load(#[from] std::io::Error),
        #[error("Failed to parse texture: {0}")]
        ParseTexture(#[from] image::ImageError),
        #[error("Failed to load \"{0}\"")]
        PackMeta(PathBuf),
        #[error("Failed to parse \"{0}\": {1}")]
        PackMetaToml(PathBuf, toml::de::Error),
        #[error("Failed to strip \"{0}\" from \"{1}\"")]
        StripPrefix(PathBuf, PathBuf),
        #[error("Invalid asset path: {0}")]
        InvalidAssetPath(PathBuf),
    }
}

#[derive(Debug, Default)]
pub struct AssetServer {
    /// All loaded packs in ascending priority
    loaded_packs: Vec<AssetPack>,
    textures: HashMap<AssetId, ImageBufferU8>,
    // TODO: Replace with signal to render thread
    pub textures_updated: bool,
}

impl AssetServer {
    async fn read_texture(raw: &[u8]) -> error::Result<ImageBufferU8> {
        let image = image::load_from_memory_with_format(raw, image::ImageFormat::Png)?;

        Ok(image.to_rgba8())
    }

    async fn load_asset(path: &Path) -> error::Result<Vec<u8>> {
        Ok(tokio::fs::read(path).await?)
    }

    pub async fn load_texture(&mut self, base: PathBuf, id: AssetId) -> error::Result<()> {
        let path = id.as_path(base, AssetType::Texture);
        let image_raw = Self::load_asset(&path).await?;
        let texture = Self::read_texture(&image_raw).await?;

        self.textures.insert(id, texture);
        self.textures_updated = true;

        Ok(())
    }

    pub fn take_textures(&mut self) -> Vec<(AssetId, ImageBufferU8)> {
        self.textures_updated = false;

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

        debug!("{:#?}", self.loaded_packs);
        debug!("{:#?}", asset_map);

        Ok(())
    }
}
