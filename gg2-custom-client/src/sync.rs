use crate::prelude::*;

#[derive(Debug)]
pub enum GameToRenderMessage {
    UpdateSpriteAtlas(TextureAtlas, ImageBufferRGBA8),
    ChangeMap(ImageBufferRGBA8),
    ExitNextFrame,
}
