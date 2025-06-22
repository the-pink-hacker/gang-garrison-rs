pub use gg2_client::networking::state::NetworkingState;
pub use gg2_custom_common::prelude::*;
pub use serde_with::skip_serializing_none;

pub use crate::{
    asset::{
        AssetServer,
        error::AssetError,
        identifier::AssetType,
        pack::AssetPack,
        sprite::{SpriteContextAsset, SpriteRenderable},
    },
    camera::Camera,
    config::ClientConfig,
    error::ClientError,
    game::ClientGame,
    init::{
        App,
        cli::{ClientCliArguments, ClientCliSubcommand},
    },
    map::MapInfo,
    networking::io::NetworkClient,
    player::ClientPlayers,
    render::{instance::SpriteInstance, texture::atlas::TextureAtlas},
    sync::GameToRenderMessage,
    world::ClientWorld,
};

pub type ImageBufferRGBA8 = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
