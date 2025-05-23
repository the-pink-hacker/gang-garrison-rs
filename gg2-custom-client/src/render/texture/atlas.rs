use std::collections::HashMap;

use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption, AtlasMipOption};

use crate::{asset::identifier::AssetId, prelude::*};

#[derive(Debug, Default)]
pub struct TextureAtlas {
    uv_lookup: HashMap<AssetId, Vec4>,
}

impl TextureAtlas {
    pub fn new(
        size: u32,
        sprites: Vec<(AssetId, ImageBufferU8)>,
    ) -> Result<(Self, ImageBufferU8), AssetError> {
        let (sprite_ids, entries) = sprites
            .into_iter()
            .map(|(id, texture)| {
                (
                    id,
                    AtlasEntry {
                        texture,
                        mip: AtlasEntryMipOption::Clamp,
                    },
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let atlas_descriptor = &AtlasDescriptor {
            max_page_count: 1,
            size,
            mip: AtlasMipOption::NoMip,
            entries: &entries,
        };

        let mut image_atlas = image_atlas::create_atlas(atlas_descriptor)?;

        use image_atlas::Texcoord32;
        let texture_coordinates = image_atlas.texcoords.into_iter().map(Texcoord32::from).map(
            |Texcoord32 {
                 min_x,
                 min_y,
                 max_x,
                 max_y,
                 ..
             }| Vec4::new(min_x, min_y, max_x - min_x, max_y - min_y),
        );
        let uv_lookup = sprite_ids.into_iter().zip(texture_coordinates).collect();

        let atlas = Self { uv_lookup };

        let texture_buffer = image_atlas
            .textures
            .pop()
            .ok_or(AssetError::AtlasEmpty)?
            .mip_maps
            .pop()
            .ok_or(AssetError::AtlasEmpty)?;

        Ok((atlas, texture_buffer))
    }

    /// A non-blocking way to get the texture coordinates of a sprite
    pub fn lookup_sprite(&self, id: &AssetId) -> Result<Vec4, AssetError> {
        Ok(*self
            .uv_lookup
            .get(id)
            .ok_or_else(|| AssetError::AtlasLookup(id.clone()))?)
    }
}
