use wgpu::*;

use crate::{
    prelude::SpriteInstance,
    render::vertex::{Vertex, VertexTextureUV},
};

#[derive(Debug)]
pub struct RenderPipelines {
    pub sprite_pipeline: RenderPipeline,
    pub map_pipeline: RenderPipeline,
    pub screen_texture_pipeline: RenderPipeline,
}

impl RenderPipelines {
    pub fn new(
        device: &Device,
        texture_bind_group_layout: &BindGroupLayout,
        camera_uniform_bind_group_layout: &BindGroupLayout,
        surface_config: &SurfaceConfiguration,
    ) -> Self {
        let sprite_shader = device.create_shader_module(include_wgsl!("shaders/sprite.wgsl"));
        let map_shader = device.create_shader_module(include_wgsl!("shaders/map.wgsl"));
        let screen_texture_shader =
            device.create_shader_module(include_wgsl!("shaders/screen_texture.wgsl"));

        let texture_camera_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Texture Camera Pipeline Layout"),
            bind_group_layouts: &[texture_bind_group_layout, camera_uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let texture_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Texture Pipeline Layout"),
            bind_group_layouts: &[texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        };

        let multisample = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let targets = &[Some(ColorTargetState {
            format: super::SCREEN_FORMAT,
            blend: Some(BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        })];

        let sprite_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Sprite Render Pipeline"),
            layout: Some(&texture_camera_layout),
            vertex: VertexState {
                module: &sprite_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout(), SpriteInstance::layout()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &sprite_shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets,
            }),
            primitive,
            depth_stencil: None,
            multisample,
            multiview: None,
            cache: None,
        });

        let map_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Map Render Pipeline"),
            layout: Some(&texture_camera_layout),
            vertex: VertexState {
                module: &map_shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[VertexTextureUV::layout()],
            },
            fragment: Some(FragmentState {
                module: &map_shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets,
            }),
            primitive,
            depth_stencil: None,
            multisample,
            multiview: None,
            cache: None,
        });

        let screen_texture_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Screen Texture Pipeline"),
            layout: Some(&texture_layout),
            vertex: VertexState {
                module: &screen_texture_shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[VertexTextureUV::layout()],
            },
            fragment: Some(FragmentState {
                module: &screen_texture_shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive,
            depth_stencil: None,
            multisample,
            multiview: None,
            cache: None,
        });

        Self {
            sprite_pipeline,
            map_pipeline,
            screen_texture_pipeline,
        }
    }
}
