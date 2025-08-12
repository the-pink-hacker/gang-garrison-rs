pub use gg2_client::networking::{
    message::server::ClientNetworkDeserializeHudMatchTimer, state::NetworkingState,
};
pub use gg2_custom_common::prelude::*;
pub use serde_with::skip_serializing_none;

pub use crate::{
    asset::{
        AssetServer,
        atlas::AtlasDefinition,
        error::AssetError,
        identifier::AssetType,
        pack::AssetPack,
        sprite::{SpriteContextAsset, SpriteRenderable},
    },
    camera::Camera,
    config::ClientConfig,
    error::ClientError,
    game::{ClientGame, gamemode::ClientGamemodeState},
    init::{
        App,
        cli::{ClientCliArguments, ClientCliSubcommand},
    },
    input::{
        InputAxisResult, InputButtonResult, InputPoll, InputState,
        bind::{InputAxisBind, InputButtonBind},
        code::{InputAxisCode, InputButtonCode},
        device::{
            InputDevice,
            gilrs::{GilrsInputState, GilrsWatcher},
            winit::{WinitInputDevice, WinitInputState},
        },
    },
    map::MapInfo,
    networking::io::NetworkClient,
    player::ClientPlayers,
    render::{instance::SpriteInstance, texture::atlas::TextureAtlas},
    sync::{ClientGameMessage, RenderMessage},
    world::ClientWorld,
};

pub type ImageBufferRGBA8 = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
