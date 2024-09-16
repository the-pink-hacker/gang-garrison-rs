use bevy::prelude::*;
use gg2_common::map::{io::MapDataLoader, MapData};

const DEBUG_MAP: &str = "maps/ctf_eiger.png";

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_data = asset_server.load::<MapData>(DEBUG_MAP);
    let map_image = asset_server.load::<Image>(DEBUG_MAP);

    commands.spawn(SpriteBundle {
        texture: map_image,
        ..default()
    });

    commands.insert_resource(LoadedMap { map_data });
}

fn debug(loaded_map: Res<LoadedMap>, maps: Res<Assets<MapData>>) {
    if let Some(map) = maps.get(&loaded_map.map_data) {
        //panic!("{:#?}", map);
    }
}

#[derive(Debug, Resource)]
pub struct LoadedMap {
    pub map_data: Handle<MapData>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<MapData>()
            .init_asset_loader::<MapDataLoader>()
            .add_systems(PreStartup, load_map)
            .add_systems(Update, debug);
    }
}
