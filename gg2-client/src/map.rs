use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use gg2_common::map::{io::MapDataLoader, MapData};

const DEBUG_MAP: &str = "maps/ctf_eiger.png";
const MAP_SCALE: f32 = 6.0;

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_data_handle = asset_server.load::<MapData>(DEBUG_MAP);
    let map_image_handle = asset_server.load::<Image>(DEBUG_MAP);

    commands.spawn((
        CurrentMap,
        SpriteBundle {
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            texture: map_image_handle,
            transform: Transform {
                scale: Vec3::splat(MAP_SCALE),
                ..default()
            },
            ..default()
        },
        Collider::default(),
    ));

    commands.insert_resource(LoadedMap {
        map_data: map_data_handle,
    });
}

fn setup_walk_collisions(
    mut current_map_query: Query<(&mut Transform, &mut Collider), With<CurrentMap>>,
    loaded_map: Res<LoadedMap>,
    mut maps: ResMut<Assets<MapData>>,
) {
    if let Ok((map_transform, mut map_collider)) = current_map_query.get_single_mut() {
        if let Some(map) = maps.remove(&loaded_map.map_data) {
            *map_collider = Collider::trimesh_with_flags(
                map.walk_mask.vertices,
                map.walk_mask.indices,
                TriMeshFlags::MERGE_DUPLICATE_VERTICES,
            );
        }
    }
}

#[derive(Component)]
pub struct CurrentMap;

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
            .add_systems(Update, setup_walk_collisions);
    }
}
