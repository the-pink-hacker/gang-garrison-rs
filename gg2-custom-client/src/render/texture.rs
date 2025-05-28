use wgpu::*;

use crate::prelude::*;

pub mod atlas;

const BITS_PER_PIXEL: u32 = 4;
const ATLAS_SIZE: u32 = 512;
const ATLAS_EXTENT: Extent3d = Extent3d {
    width: ATLAS_SIZE,
    height: ATLAS_SIZE,
    depth_or_array_layers: 1,
};

#[derive(Debug)]
pub struct RenderTextures {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
    pub sprite_atlas_bind_group: BindGroup,
    pub sprite_atlas_texture: Texture,
    pub sprite_atlas: TextureAtlas,
    pub map_bind_group: Option<BindGroup>,
}

impl RenderTextures {
    pub fn new(device: &wgpu::Device) -> Result<Self, ClientError> {
        let sprite_atlas_texture = device.create_texture(&TextureDescriptor {
            label: Some("Sprite Atlas Texture"),
            size: ATLAS_EXTENT,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let sprite_atlas_view = sprite_atlas_texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Diffuse Sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let sprite_atlas_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&sprite_atlas_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });
        let sprite_atlas = TextureAtlas::default();

        Ok(Self {
            layout,
            sampler,
            sprite_atlas_bind_group,
            sprite_atlas_texture,
            map_bind_group: None,
            sprite_atlas,
        })
    }

    pub fn update_texture_map(&mut self, device: &Device, queue: &Queue, image: ImageBufferRGBA8) {
        debug!("Updating map texture buffer...");

        let width = image.width();
        let height = image.height();

        let size = Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        let map_texture = device.create_texture(&TextureDescriptor {
            label: Some("Map Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let map_view = map_texture.create_view(&TextureViewDescriptor::default());

        let map_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &self.layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&map_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &map_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &image,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(BITS_PER_PIXEL * width),
                rows_per_image: Some(height),
            },
            size,
        );

        self.map_bind_group = Some(map_bind_group);
    }

    pub fn update_texture_atlas(
        &mut self,
        queue: &Queue,
        textures: Vec<(AssetId, ImageBufferRGBA8)>,
    ) {
        debug!("Updating texture atlas buffer...");
        let (texture_atlas, diffuse_rgba) =
            TextureAtlas::new(ATLAS_SIZE, textures).expect("Failed to construct texture atlas");

        self.sprite_atlas = texture_atlas;

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &self.sprite_atlas_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &diffuse_rgba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(BITS_PER_PIXEL * ATLAS_SIZE),
                rows_per_image: Some(ATLAS_SIZE),
            },
            ATLAS_EXTENT,
        );
    }
}
