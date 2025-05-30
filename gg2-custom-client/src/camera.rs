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

impl ClientGame {
    pub async fn update_camera(&self) -> Result<(), ClientError> {
        if let Ok(player) = self
            .world
            .players()
            .read()
            .await
            .get(PlayerId::from_u8(0).unwrap())
        {
            self.world.camera().write().await.translation = player.transform.translation.xy();
        }

        Ok(())
    }
}
