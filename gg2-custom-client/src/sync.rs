use crate::prelude::*;

#[derive(Debug)]
pub enum GameToRenderMessage {
    UpdateSpriteAtlas(Vec<(ResourceId, ImageBufferRGBA8)>),
    ChangeMap(ImageBufferRGBA8),
    ExitNextFrame,
}
