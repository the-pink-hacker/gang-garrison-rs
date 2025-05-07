use crate::prelude::*;

/// Where the game is rendered from
#[derive(Debug)]
pub struct Camera {
    /// The location
    pub translation: Vec2,
    /// The near clipping plane's z
    pub clipping_near: f32,
    /// The far clipping plane's z
    pub clipping_far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            clipping_near: -100.0,
            clipping_far: 100.0,
        }
    }
}

impl UpdateMutRunnable for Camera {
    async fn update_mut(&mut self, _world: &World) -> Result<()> {
        // Move camera up and to the right
        //self.translation.x += 0.01;
        //self.translation.y -= 0.01;

        Ok(())
    }
}
