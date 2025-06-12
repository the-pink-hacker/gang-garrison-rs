use crate::prelude::*;

#[derive(Debug)]
pub enum GameToRenderMessage {
    UpdateSpriteAtlas(Vec<(AssetId, ImageBufferRGBA8)>),
    ChangeMap(ImageBufferRGBA8),
    ExitNextFrame,
}
