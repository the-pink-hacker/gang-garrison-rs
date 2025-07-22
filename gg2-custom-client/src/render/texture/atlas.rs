use std::collections::HashMap;

use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption, AtlasMipOption};

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct TextureAtlas {
    uv_lookup: HashMap<ResourceId, Vec<Vec4>>,
    pub size: u32,
}

impl TextureAtlas {
    pub fn new(
        size: u32,
        mut textures: Vec<(ResourceId, ImageBufferRGBA8)>,
    ) -> Result<(Self, ImageBufferRGBA8), AssetError> {
        // Sort to keep animations in order
        textures.sort_by(|(id, _), (other_id, _)| id.cmp(other_id));

        let (texture_ids, entries) = textures
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

        let mut atlas = Self {
            uv_lookup: Default::default(),
            size,
        };

        for (mut id, texture_coordinate) in texture_ids.into_iter().zip(texture_coordinates) {
            let is_animation = id
                .file_name()
                .map(str::parse::<usize>)
                .and_then(Result::ok)
                .is_some();

            if is_animation {
                id.pop();
            }

            atlas
                .uv_lookup
                .entry(id)
                .or_default()
                .push(texture_coordinate);
        }

        let texture_buffer = image_atlas
            .textures
            .pop()
            .ok_or(AssetError::AtlasEmpty)?
            .mip_maps
            .pop()
            .ok_or(AssetError::AtlasEmpty)?;

        Ok((atlas, texture_buffer))
    }

    pub fn lookup_sprite(&self, id: &ResourceId) -> Result<Vec4, AssetError> {
        Ok(*self
            .lookup_sprite_many(id)?
            .first()
            .unwrap_or_else(|| panic!("Sprite {id} has no frames")))
    }

    pub fn lookup_sprite_many(&self, id: &ResourceId) -> Result<&[Vec4], AssetError> {
        Ok(self
            .uv_lookup
            .get(id)
            .ok_or_else(|| AssetError::AtlasLookup(id.clone()))?)
    }
}
