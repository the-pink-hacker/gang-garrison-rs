// This resource helped a lot:
// https://sotrh.github.io/learn-wgpu/

use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::run_on_demand::EventLoopExtRunOnDemand,
    window::{Window, WindowId},
};

use crate::{
    init::{App, World},
    prelude::*,
};
use instance::Instance;
use vertex::Vertex;

pub mod camera;
pub mod instance;
pub mod texture;
pub mod vertex;

const MAX_SPRITE_INSTANCES: wgpu::BufferAddress = 1_024;

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
pub struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    texture_bind_group: wgpu::BindGroup,
    texture_atlas: wgpu::Texture,
    /// Store the camera's matrix
    camera_uniform_buffer: wgpu::Buffer,
    camera_uniform_bind_group: wgpu::BindGroup,
    sprite_instances: Vec<Instance>,
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("render/shaders/shader.wgsl"));

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let sprite_instances = vec![Instance::from_transform_origin(
            Transform {
                translation: Vec3::ZERO,
                rotation: 0.0,
                scale: Vec2::splat(128.0),
            },
            Vec2::new(0.5, 0.5),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
        )];

        let sprite_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Instance Buffer"),
            size: std::mem::size_of::<Instance>() as wgpu::BufferAddress * MAX_SPRITE_INSTANCES,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let (camera_uniform_bind_group_layout, camera_uniform_bind_group, camera_uniform_buffer) =
            Self::create_camera_buffer(&device);

        let (texture_bind_group_layout, texture_bind_group, texture_atlas) =
            Self::create_texture_bind_group(&device)?;

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_uniform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout(), Instance::layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            texture_bind_group,
            texture_atlas,
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

    async fn render(&mut self, world: &World) {
        self.update_camera_uniform_buffer(world).await;
        self.update_texture_atlas(world).await;

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

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_uniform_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.sprite_instance_buffer.slice(..));

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

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
    pub fn init_render(&self) -> Result<(), ClientError> {
        let mut event_loop = EventLoop::new()?;

        event_loop.set_control_flow(ControlFlow::Wait);

        event_loop
            .run_app_on_demand(&mut RenderApp::new(self.get_world()))
            .unwrap();

        Ok(())
    }
}

pub struct RenderApp {
    world: Arc<World>,
    state: Option<State>,
}

impl RenderApp {
    fn new(world: Arc<World>) -> Self {
        Self { world, state: None }
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

        let state = pollster::block_on(State::new(Arc::clone(&window)))
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
        let state = self.state.as_mut().expect("Render state is uninitialized");
        match event {
            WindowEvent::CloseRequested => {
                info!("User closed window; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                pollster::block_on(state.render(&self.world));

                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => state.resize(size),
            _ => (),
        }
    }
}
