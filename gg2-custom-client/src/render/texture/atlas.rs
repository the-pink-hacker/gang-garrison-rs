use std::path::{Path, PathBuf};

use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption, AtlasMipOption};

use crate::prelude::*;

fn load_image(path: impl AsRef<Path>) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    let path = path.as_ref();
    let diffuse_bytes =
        std::fs::read(path).map_err(|_| Error::AssetLoad(path.display().to_string()))?;

    let diffuse_image =
        image::load_from_memory_with_format(&diffuse_bytes, image::ImageFormat::Png)
            .map_err(|_| Error::AssetLoad(path.display().to_string()))?;

    Ok(diffuse_image.to_rgba8())
}

pub fn create_atlas(
    size: u32,
    images: &[PathBuf],
) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    let mut entries = Vec::with_capacity(images.len());

    for image in images {
        let entry = AtlasEntry {
            texture: load_image(image)?,
            mip: AtlasEntryMipOption::Clamp,
        };

        entries.push(entry);
    }

    let atlas = image_atlas::create_atlas(&AtlasDescriptor {
        max_page_count: 1,
        size,
        mip: AtlasMipOption::NoMip,
        entries: &entries,
    })?;

    Ok(atlas.textures[0].mip_maps[0].clone())
}
