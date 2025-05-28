pub use gg2_custom_common::prelude::*;
pub use serde_with::skip_serializing_none;

pub use crate::{
    asset::{
        AssetServer,
        error::AssetError,
        identifier::{AssetId, AssetType},
        pack::AssetPack,
        sprite::{SpriteContextAsset, SpriteRenderable},
    },
    camera::Camera,
    config::{ClientConfig, ClientConfigLock},
    error::ClientError,
    init::{
        App, UpdateMutRunnable,
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
