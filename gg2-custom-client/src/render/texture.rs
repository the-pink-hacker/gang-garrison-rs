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
    pub game_bind_group: BindGroup,
    pub game_texture: Texture,
    pub depth_texture: Texture,
}

impl RenderTextures {
    pub fn new(device: &wgpu::Device, game_size: UVec2) -> Result<Self, ClientError> {
        let sprite_atlas_texture = device.create_texture(&TextureDescriptor {
            label: Some("Sprite Atlas Texture"),
            size: ATLAS_EXTENT,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: super::SCREEN_FORMAT,
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
            label: Some("Sprite Atlas Bind Group"),
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

        let (game_bind_group, game_texture) =
            Self::generate_game_texture(device, &layout, &sampler, game_size);

        let depth_texture = Self::generate_depth_texture(device, game_size);

        Ok(Self {
            layout,
            sampler,
            sprite_atlas_bind_group,
            sprite_atlas_texture,
            map_bind_group: None,
            sprite_atlas,
            game_bind_group,
            game_texture,
            depth_texture,
        })
    }

    fn generate_game_texture(
        device: &Device,
        layout: &BindGroupLayout,
        sampler: &Sampler,
        game_size: UVec2,
    ) -> (BindGroup, Texture) {
        let game_texture = device.create_texture(&TextureDescriptor {
            label: Some("Game Texture"),
            size: Extent3d {
                width: game_size.x,
                height: game_size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: super::SCREEN_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[super::SCREEN_FORMAT.add_srgb_suffix()],
        });

        let game_texture_view = game_texture.create_view(&TextureViewDescriptor {
            format: Some(super::SCREEN_FORMAT.add_srgb_suffix()),
            ..Default::default()
        });

        let game_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Game Texture Bind Group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&game_texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
        });

        (game_bind_group, game_texture)
    }

    fn generate_depth_texture(device: &Device, game_size: UVec2) -> Texture {
        device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: game_size.x,
                height: game_size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: super::DEPTH_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
    }

    pub fn update_game_texture(&mut self, device: &Device, game_size: UVec2) {
        let (bind_group, texture) =
            Self::generate_game_texture(device, &self.layout, &self.sampler, game_size);

        let depth_texture = Self::generate_depth_texture(device, game_size);

        self.game_bind_group = bind_group;
        self.game_texture = texture;
        self.depth_texture = depth_texture;
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
            format: super::SCREEN_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let map_view = map_texture.create_view(&TextureViewDescriptor::default());

        let map_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Map Bind Group"),
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
        textures: Vec<(ResourceId, ImageBufferRGBA8)>,
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
