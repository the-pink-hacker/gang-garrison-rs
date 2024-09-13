use bevy::{prelude::*, render::texture::ImageLoader};
use gg2_common::map::{io::MapDataLoader, MapData};

const DEBUG_MAP: &str = "maps/ctf_eiger.png";

fn load_map(asset_server: Res<AssetServer>) {
    let map_data_handle = asset_server.load::<MapData>(DEBUG_MAP);
    let map_image_handle = asset_server.load::<Image>(DEBUG_MAP);
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<MapData>()
            .init_asset_loader::<MapDataLoader>()
            .init_asset::<Image>()
            .init_asset_loader::<ImageLoader>()
            .add_systems(PreStartup, load_map);
    }
}
