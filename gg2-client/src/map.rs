use std::path::PathBuf;

use bevy::prelude::*;
use gg2_common::{map::*, networking::message::ServerChangeMap};

use crate::networking::NetworkData;

fn load_map_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_state: ResMut<NextState<MapLoadState>>,
    mut change_map_event: EventReader<NetworkData<ServerChangeMap>>,
) {
    for map_event in change_map_event.read() {
        let map_path = PathBuf::from("maps").join(format!("{}.png", map_event.map_name));
        let map_data_handle = asset_server.load::<MapData>(map_path.clone());
        let map_image_handle = asset_server.load::<Image>(map_path);

        commands.spawn((
            CommonMapBundle::from_handle(map_data_handle),
            Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                image: map_image_handle,
                ..default()
            },
        ));

        load_state.set(MapLoadState::Loading);
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommonMapPlugin).add_systems(
            FixedUpdate,
            load_map_system
                .run_if(in_state(MapLoadState::Unloaded).or(in_state(MapLoadState::Loaded)))
                .run_if(on_event::<NetworkData<ServerChangeMap>>),
        );
    }
}
