use glam::Quat;

use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transform {
    pub translation: Vec3,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Transform {
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    /// An origin of `0, 0` is at the bottom left corner
    pub fn calculate_matrix_origin(&self, origin: Vec2) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale.extend(1.0),
            Quat::from_rotation_z(self.rotation),
            Vec3::new(
                self.translation.x - origin.x * self.scale.x,
                -self.translation.y - origin.y * self.scale.y,
                self.translation.z,
            ),
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}
