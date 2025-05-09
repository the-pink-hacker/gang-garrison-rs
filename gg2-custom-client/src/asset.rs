use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use identifier::{AssetId, AssetType};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub mod identifier;

pub mod error {
    pub type Result<T> = std::result::Result<T, AssetError>;

    #[derive(Debug, thiserror::Error)]
    pub enum AssetError {
        #[error("Failed to load asset: (0)")]
        Load(#[from] std::io::Error),
        #[error("Failed to parse texture: (0)")]
        ParseTexture(#[from] image::ImageError),
    }
}

#[derive(Debug, Default)]
pub struct AssetServer {
    textures: HashMap<AssetId, ImageBufferU8>,
    // TODO: Replace with signal to render thread
    pub textures_updated: bool,
}

impl AssetServer {
    async fn read_texture(raw: &[u8]) -> error::Result<ImageBufferU8> {
        let image = image::load_from_memory_with_format(raw, image::ImageFormat::Png)?;

        Ok(image.to_rgba8())
    }

    async fn load_asset(path: impl AsRef<Path>) -> error::Result<Vec<u8>> {
        Ok(tokio::fs::read(path).await?)
    }

    pub async fn load_texture(&mut self, base: PathBuf, id: AssetId) -> error::Result<()> {
        let path = id.as_path(base, AssetType::Texture);
        let image_raw = Self::load_asset(path).await?;
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetPackMetadataRoot {
    pack: AssetPackMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetPackMetadata {
    name: String,
    description: String,
    liscense: String,
    version: String,
    format: u8,
}
