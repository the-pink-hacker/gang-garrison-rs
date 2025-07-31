use crate::prelude::*;

#[derive(Debug)]
pub enum RenderMessage {
    UpdateSpriteAtlas(TextureAtlas, ImageBufferRGBA8),
    ChangeMap(ImageBufferRGBA8),
    ExitNextFrame,
}

#[derive(Debug)]
pub enum ClientGameMessage {
    GilrsEvent(gilrs::Event),
}
