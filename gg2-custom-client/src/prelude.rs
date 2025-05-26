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
    error::ClientError,
    init::{App, UpdateMutRunnable, World, cli::ClientCliSubcommand},
    map::MapInfo,
    networking::io::NetworkClient,
    player::{Player, Players},
    render::{instance::SpriteInstance, texture::atlas::TextureAtlas},
    sync::GameToRenderMessage,
};

pub type ImageBufferRGBA8 = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
