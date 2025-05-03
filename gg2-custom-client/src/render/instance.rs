use crate::prelude::*;

/// Stores information about a sprite
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    transform_matrix: Mat4,
    /// Where x and y are the top left position, and z and w are the size
    texture_uv: Vec4,
}

impl Instance {
    const ATTRIBUTES: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        // Transform Matrix
        1 => Float32x4,
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        // Texture UV
        5 => Float32x4,
    ];

    /// An origin of `0, 0` is at the bottom left corner
    pub fn from_transform_origin(transform: Transform, origin: Vec2, texture_uv: Vec4) -> Self {
        Self {
            transform_matrix: transform.calculate_matrix_origin(origin),
            texture_uv,
        }
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}
