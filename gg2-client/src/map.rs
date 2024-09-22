use bevy::prelude::*;
use gg2_common::map::*;

const DEBUG_MAP: &str = "maps/cp_dirtbowl.png";

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_state: ResMut<NextState<MapLoadState>>,
) {
    let map_data_handle = asset_server.load::<MapData>(DEBUG_MAP);
    let map_image_handle = asset_server.load::<Image>(DEBUG_MAP);

    commands.spawn((
        CommonMapBundle::from_handle(map_data_handle),
        Sprite {
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        map_image_handle,
    ));

    load_state.set(MapLoadState::Loading);
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommonMapPlugin).add_systems(
            FixedUpdate,
            load_map.run_if(in_state(MapLoadState::Unloaded)),
        );
    }
}
