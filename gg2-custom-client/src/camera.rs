use crate::prelude::*;

const GAMEMAKER_CONVERSION: Vec3 = Vec3::new(1.0, -1.0, 1.0);

/// Where the game is rendered from
#[derive(Debug)]
pub struct Camera {
    /// The location
    pub translation: Vec3,
    /// The near clipping plane's distance from the camera
    pub clipping_near: f32,
    /// The far clipping plane's distance from the camera
    pub clipping_far: f32,
}

impl Camera {
    /// Genrates a matrix to project world space into screen space
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        //let projection = Mat4::orthographic_rh_gl(
        //    -50.0,
        //    50.0,
        //    -50.0,
        //    50.0,
        //    self.clipping_near,
        //    self.clipping_far,
        //);
        // At the moment can't seem to get orthographic working
        let projection = Mat4::perspective_rh_gl(1.0, 16.0 / 9.0, 0.1, 100.0);

        let translation_converted = self.translation * GAMEMAKER_CONVERSION;

        let view = Mat4::look_at_rh(
            translation_converted,
            translation_converted.with_z(0.0),
            Vec3::Y,
        );

        use glam::Vec4;

        projection * view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            translation: Vec3::new(0.0, 0.0, 10.0),
            clipping_near: 0.1,
            clipping_far: 100.0,
        }
    }
}

impl UpdateMutRunnable for Camera {
    async fn update_mut(&mut self, world: &World) -> Result<()> {
        // Move camera up and to the right
        self.translation.x += 0.01;
        self.translation.y -= 0.01;

        Ok(())
    }
}
