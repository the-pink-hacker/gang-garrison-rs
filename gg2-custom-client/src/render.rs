// This resource helped a lot:
// https://sotrh.github.io/learn-wgpu/

use std::sync::Arc;

use tokio::sync::mpsc::UnboundedReceiver;
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::prelude::*;
use vertex::{Vertex, VertexTextureUV};

pub mod camera;
pub mod instance;
pub mod pipeline;
pub mod texture;
pub mod vertex;

const GAME_ASPECT_RATIO: UVec2 = UVec2::new(4, 3);
pub const GAME_WIDTH: u32 = 800;
pub const GAME_HEIGHT: u32 = 600;

const MAX_SPRITE_INSTANCES: wgpu::BufferAddress = u16::MAX as wgpu::BufferAddress;
const MAP_SCALE: f32 = 6.0;

const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: Vec2::new(1.0, 1.0),
    },
    Vertex {
        position: Vec2::new(0.0, 1.0),
    },
    Vertex {
        position: Vec2::new(0.0, 0.0),
    },
    Vertex {
        position: Vec2::new(1.0, 0.0),
    },
];

#[rustfmt::skip]
const QUAD_INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3
];

/// Holds all rendering structs such as the window
#[derive(Debug)]
pub struct State {
    world: &'static ClientWorld,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    pipelines: pipeline::RenderPipelines,
    sprite_vertex_buffer: wgpu::Buffer,
    map_vertex_buffer: wgpu::Buffer,
    game_vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    textures: texture::RenderTextures,
    /// Store the camera's matrix
    camera_uniform_buffer: wgpu::Buffer,
    camera_uniform_bind_group: wgpu::BindGroup,
    sprite_instances: Vec<SpriteInstance>,
    sprite_instance_buffer: wgpu::Buffer,
}

impl State {
    async fn new(window: Arc<Window>, world: &'static ClientWorld) -> Result<State, ClientError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;

        let size = window.inner_size();
        let window_size = UVec2::new(size.width, size.height);
        let game_size = Self::calculate_game_size(window_size, GAME_ASPECT_RATIO);

        let surface = instance.create_surface(Arc::clone(&window))?;
        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities.formats[0];

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            view_formats: vec![surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::Immediate,
        };

        let sprite_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let map_vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Map Vertex Buffer"),
            size: std::mem::size_of::<VertexTextureUV>() as wgpu::BufferAddress * 4,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let game_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Game Vertex Buffer"),
            contents: bytemuck::cast_slice(&Self::calculate_game_vertex(window_size, game_size)),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let sprite_instances = Vec::default();
        let sprite_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Instance Buffer"),
            size: std::mem::size_of::<SpriteInstance>() as wgpu::BufferAddress
                * MAX_SPRITE_INSTANCES,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let (camera_uniform_bind_group_layout, camera_uniform_bind_group, camera_uniform_buffer) =
            Self::create_camera_buffer(&device);

        let textures = texture::RenderTextures::new(&device, UVec2::new(size.width, size.height))?;

        let pipelines = pipeline::RenderPipelines::new(
            &device,
            &textures.layout,
            &camera_uniform_bind_group_layout,
            &surface_config,
        );

        let state = State {
            world,
            window,
            device,
            queue,
            size,
            surface,
            surface_config,
            pipelines,
            sprite_vertex_buffer,
            map_vertex_buffer,
            game_vertex_buffer,
            index_buffer,
            textures,
            camera_uniform_buffer,
            camera_uniform_bind_group,
            sprite_instances,
            sprite_instance_buffer,
        };

        state.configure_surface();

        Ok(state)
    }

    // I'm not fucking rewriting this: 35250c0fa8b5067df7efb5cd603047a228e5fd49
    fn calculate_game_size(window_size: UVec2, aspect_ratio: UVec2) -> UVec2 {
        let width =
            ((window_size.y as f32 / aspect_ratio.y as f32) * aspect_ratio.x as f32).trunc() as u32;

        if width > window_size.x {
            let height = (window_size.x as f32 / aspect_ratio.x as f32) * aspect_ratio.y as f32;
            UVec2::new(window_size.x, height.trunc() as u32)
        } else {
            UVec2::new(width, window_size.y)
        }
    }

    fn calculate_game_vertex(window_size: UVec2, game_size: UVec2) -> [VertexTextureUV; 4] {
        let half_game_normalized = game_size.as_vec2() / window_size.as_vec2();

        [
            VertexTextureUV {
                position: half_game_normalized,
                texture_uv: Vec2::new(1.0, 0.0),
            },
            VertexTextureUV {
                position: Vec2::new(-half_game_normalized.x, half_game_normalized.y),
                texture_uv: Vec2::new(0.0, 0.0),
            },
            VertexTextureUV {
                position: -half_game_normalized,
                texture_uv: Vec2::new(0.0, 1.0),
            },
            VertexTextureUV {
                position: Vec2::new(half_game_normalized.x, -half_game_normalized.y),
                texture_uv: Vec2::new(1.0, 1.0),
            },
        ]
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.surface_config.width = self.size.width;
        self.surface_config.height = self.size.height;
        self.configure_surface();

        let window_size = UVec2::new(self.size.width, self.size.height);
        let game_size = Self::calculate_game_size(window_size, GAME_ASPECT_RATIO);
        let game_vertices = Self::calculate_game_vertex(window_size, game_size);

        self.queue.write_buffer(
            &self.game_vertex_buffer,
            0,
            bytemuck::cast_slice(&game_vertices),
        );

        self.textures.update_game_texture(&self.device, game_size);
    }

    async fn render(
        &mut self,
        game_to_render_channel: &mut UnboundedReceiver<GameToRenderMessage>,
    ) {
        self.update_camera_uniform_buffer().await;

        if let Ok(message) = game_to_render_channel.try_recv() {
            match message {
                GameToRenderMessage::UpdateSpriteAtlas(textures) => {
                    self.textures.update_texture_atlas(&self.queue, textures);
                }
                GameToRenderMessage::ChangeMap(image) => {
                    let width = image.width() as f32 * MAP_SCALE;
                    let height = image.height() as f32 * MAP_SCALE;

                    self.queue.write_buffer(
                        &self.map_vertex_buffer,
                        0,
                        bytemuck::cast_slice(&[
                            VertexTextureUV {
                                position: Vec2::new(width, -height),
                                texture_uv: Vec2::new(1.0, 1.0),
                            },
                            VertexTextureUV {
                                position: Vec2::new(0.0, -height),
                                texture_uv: Vec2::new(0.0, 1.0),
                            },
                            VertexTextureUV {
                                position: Vec2::new(0.0, 0.0),
                                texture_uv: Vec2::new(0.0, 0.0),
                            },
                            VertexTextureUV {
                                position: Vec2::new(width, 0.0),
                                texture_uv: Vec2::new(1.0, 0.0),
                            },
                        ]),
                    );

                    self.textures
                        .update_texture_map(&self.device, &self.queue, image);
                }
            }
        }

        self.sprite_instances = {
            let players = self.world.players().read().await;
            let asset_server = self.world.asset_server().read().await;

            players
                .iter()
                .map(|player| player.render(&self.textures.sprite_atlas, &asset_server))
                .flat_map(|player| match player {
                    Ok(player) => player,
                    Err(error) => {
                        error!("Sprite Render: {error}");
                        None
                    }
                })
                .collect()
        };

        // ATLAS TEST
        self.sprite_instances
            .push(SpriteInstance::from_transform_origin(
                Transform {
                    translation: Vec3::new(1142.0, 504.4, -0.1),
                    rotation: 0.0,
                    scale: Vec2::splat(128.0),
                },
                Vec2::new(0.5, 0.5),
                Vec4::new(0.0, 0.0, 1.0, 1.0),
            ));

        // Maybe this should be a compute shader?
        // Sort by z
        self.sprite_instances.sort_by(|instance, other_instance| {
            instance
                .translation_z()
                .total_cmp(&other_instance.translation_z())
        });

        self.queue.write_buffer(
            &self.sprite_instance_buffer,
            0,
            bytemuck::cast_slice(&self.sprite_instances),
        );

        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(self.surface_config.format.add_srgb_suffix()),
            ..Default::default()
        };
        let game_view = self
            .textures
            .game_texture
            .create_view(&texture_view_descriptor);

        let mut encoder = self.device.create_command_encoder(&Default::default());
        // Game View Pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &game_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_bind_group(1, &self.camera_uniform_bind_group, &[]);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            if let Some(map_bind_group) = &self.textures.map_bind_group {
                render_pass.set_bind_group(0, map_bind_group, &[]);
                render_pass.set_pipeline(&self.pipelines.map_pipeline);
                render_pass.set_vertex_buffer(0, self.map_vertex_buffer.slice(..));

                render_pass.draw_indexed(0..(QUAD_INDICES.len() as u32), 0, 0..1);
            }

            render_pass.set_bind_group(0, &self.textures.sprite_atlas_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.sprite_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.sprite_instance_buffer.slice(..));
            render_pass.set_pipeline(&self.pipelines.sprite_pipeline);

            render_pass.draw_indexed(
                0..(QUAD_INDICES.len() as u32),
                0,
                0..(self.sprite_instances.len() as u32),
            );
        }

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swapchain texture");
        let surface_view = surface_texture
            .texture
            .create_view(&texture_view_descriptor);

        // Final Window Pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Aspect Ratio Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.set_bind_group(0, &self.textures.game_bind_group, &[]);
            render_pass.set_pipeline(&self.pipelines.screen_texture_pipeline);
            render_pass.set_vertex_buffer(0, self.game_vertex_buffer.slice(..));

            render_pass.draw_indexed(0..(QUAD_INDICES.len() as u32), 0, 0..1);
        }

        // Submit and queue the command
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

impl App {
    /// Initializes render loop
    pub fn init_render(self, runtime: tokio::runtime::Runtime) -> Result<(), ClientError> {
        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(ControlFlow::Wait);

        event_loop.run_app(&mut RenderApp::new(
            self.world,
            runtime,
            self.game_to_render_channel_receiver,
        ))?;

        Ok(())
    }
}

pub struct RenderApp {
    world: &'static ClientWorld,
    state: Option<State>,
    runtime: tokio::runtime::Runtime,
    game_to_render_channel: UnboundedReceiver<GameToRenderMessage>,
}

impl RenderApp {
    fn new(
        world: &'static ClientWorld,
        runtime: tokio::runtime::Runtime,
        game_to_render_channel: UnboundedReceiver<GameToRenderMessage>,
    ) -> Self {
        Self {
            world,
            state: None,
            runtime,
            game_to_render_channel,
        }
    }
}

impl ApplicationHandler for RenderApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .expect("Failed to create window"),
        );

        let state = self
            .runtime
            .block_on(State::new(Arc::clone(&window), self.world))
            .expect("Failed to create render state");

        self.state = Some(state);
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().expect("Render state uninitilized");

        match event {
            WindowEvent::CloseRequested => {
                info!("User closed window; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.runtime.block_on(async {
                    state.render(&mut self.game_to_render_channel).await;

                    state.get_window().request_redraw();
                });
            }
            WindowEvent::Resized(size) => {
                self.runtime.block_on(async {
                    state.resize(size);
                });
            }
            _ => (),
        }
    }
}
