use super::State;
use crate::prelude::*;

const GAME_HEIGHT: u32 = 128 * 6;

impl Camera {
    /// Genrates a matrix to project world space into screen space
    fn build_view_projection_matrix(&self, window_size: UVec2) -> Mat4 {
        // TODO: Add aspect ratio crop
        let ratio = window_size.x as f32 / window_size.y as f32;
        let width = (GAME_HEIGHT as f32 * ratio).trunc();
        let game_size = Vec2::new(width, GAME_HEIGHT as f32);
        let (width_half, height_half) = (game_size / 2.0).into();

        Mat4::orthographic_rh_gl(
            self.translation.x - width_half,
            self.translation.x + width_half,
            -self.translation.y - height_half,
            -self.translation.y + height_half,
            self.clipping_near,
            self.clipping_far,
        )
    }
}

impl State {
    pub fn create_camera_buffer(
        device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup, wgpu::Buffer) {
        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Uniform Bind Group Layout"),
            layout: &camera_uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        (
            camera_uniform_bind_group_layout,
            camera_uniform_bind_group,
            camera_uniform_buffer,
        )
    }

    pub async fn update_camera_uniform_buffer(&mut self, world: &ClientWorld) {
        let camera = world.camera().read().await;
        let matrix =
            camera.build_view_projection_matrix(UVec2::new(self.size.width, self.size.height));
        self.queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&[matrix]),
        );
    }
}
