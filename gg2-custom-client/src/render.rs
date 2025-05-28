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

const MAX_SPRITE_INSTANCES: wgpu::BufferAddress = u16::MAX as wgpu::BufferAddress;
const MAP_SCALE: f32 = 6.0;

const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: Vec3::new(1.0, 1.0, 0.0),
    },
    Vertex {
        position: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        position: Vec3::new(0.0, 0.0, 0.0),
    },
    Vertex {
        position: Vec3::new(1.0, 0.0, 0.0),
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
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    pipelines: pipeline::RenderPipelines,
    sprite_vertex_buffer: wgpu::Buffer,
    map_vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    textures: texture::RenderTextures,
    /// Store the camera's matrix
    camera_uniform_buffer: wgpu::Buffer,
    camera_uniform_bind_group: wgpu::BindGroup,
    sprite_instances: Vec<SpriteInstance>,
    sprite_instance_buffer: wgpu::Buffer,
}

impl State {
    async fn new(window: Arc<Window>) -> Result<State, ClientError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;

        let size = window.inner_size();

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

        let textures = texture::RenderTextures::new(&device)?;

        let pipelines = pipeline::RenderPipelines::new(
            &device,
            &textures.layout,
            &camera_uniform_bind_group_layout,
            &surface_config,
        );

        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_config,
            pipelines,
            sprite_vertex_buffer,
            map_vertex_buffer,
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
    }

    async fn render(
        &mut self,
        world: &World,
        game_to_render_channel: &mut UnboundedReceiver<GameToRenderMessage>,
    ) {
        self.update_camera_uniform_buffer(world).await;

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
                                position: Vec3::new(width, -height, 0.0),
                                texture_uv: Vec2::new(1.0, 1.0),
                            },
                            VertexTextureUV {
                                position: Vec3::new(0.0, -height, 0.0),
                                texture_uv: Vec2::new(0.0, 1.0),
                            },
                            VertexTextureUV {
                                position: Vec3::new(0.0, 0.0, 0.0),
                                texture_uv: Vec2::new(0.0, 0.0),
                            },
                            VertexTextureUV {
                                position: Vec3::new(width, 0.0, 0.0),
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
            let players = world.players.read().await;
            let asset_server = world.asset_server.read().await;

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

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_config.format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());
        // Render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
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
            self.get_world(),
            runtime,
            self.game_to_render_channel_receiver,
        ))?;

        Ok(())
    }
}

pub struct RenderApp {
    world: Arc<World>,
    state: Option<State>,
    runtime: tokio::runtime::Runtime,
    game_to_render_channel: UnboundedReceiver<GameToRenderMessage>,
}

impl RenderApp {
    fn new(
        world: Arc<World>,
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
            .block_on(State::new(Arc::clone(&window)))
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
                    state
                        .render(&self.world, &mut self.game_to_render_channel)
                        .await;

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
