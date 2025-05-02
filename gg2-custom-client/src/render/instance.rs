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

    pub fn from_translation(translation: Vec3, texture_uv: Vec4) -> Self {
        let transform_matrix = Mat4::from_translation(translation);

        Self {
            transform_matrix,
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
