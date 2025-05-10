pub use gg2_custom_common::prelude::*;
pub use serde_with::skip_serializing_none;

pub use crate::{
    asset::{
        AssetServer,
        error::AssetError,
        identifier::{AssetId, AssetType},
        pack::AssetPack,
    },
    camera::Camera,
    error::ClientError,
    init::{UpdateMutRunnable, World, cli::ClientCliSubcommand},
    networking::io::NetworkClient,
};

pub type ImageBufferU8 = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
