use crate::prelude::*;

#[derive(Debug, Default)]
pub struct MapInfo {
    pub current_map: Option<(ResourceId, MapData)>,
}

impl ClientGame {
    pub async fn event_map_change(&self, message: ServerChangeMap) -> Result<(), ClientError> {
        let map_id = ResourceId::gg2((*message.map_name).clone());
        info!("Map loading: {map_id}");

        let (image, data) = self
            .world
            .asset_server()
            .read()
            .await
            .load_map(&map_id)
            .await?;

        self.world.map_info().write().await.current_map = Some((map_id.clone(), data));

        self.world
            .game_to_render_channel()
            .send(GameToRenderMessage::ChangeMap(image))?;

        Ok(())
    }
}
