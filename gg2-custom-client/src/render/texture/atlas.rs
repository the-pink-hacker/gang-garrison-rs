use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption, AtlasMipOption};

use crate::{asset::identifier::AssetId, prelude::*};

pub fn create_atlas(
    size: u32,
    textures: Vec<(AssetId, ImageBufferU8)>,
) -> Result<ImageBufferU8, ClientError> {
    let entries = textures
        .into_iter()
        .map(|(_id, texture)| AtlasEntry {
            texture,
            mip: AtlasEntryMipOption::Clamp,
        })
        .collect::<Vec<_>>();

    let atlas = image_atlas::create_atlas(&AtlasDescriptor {
        max_page_count: 1,
        size,
        mip: AtlasMipOption::NoMip,
        entries: &entries,
    })?;

    Ok(atlas.textures[0].mip_maps[0].clone())
}
