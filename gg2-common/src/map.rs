use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use collision::mesh::WalkMeshMask;
use io::MapDataLoader;

use crate::entity::entities::MapEntity;

pub mod collision;
pub mod io;

const MAP_SCALE: f32 = 6.0;

#[derive(Debug, Asset, TypePath)]
pub struct MapData {
    pub entities: Vec<MapEntity>,
    pub walk_mask: WalkMeshMask,
}

#[derive(Component, Default)]
pub struct CurrentMap;

#[derive(Bundle)]
pub struct CommonMapBundle {
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub map_data: Handle<MapData>,
    current_map: CurrentMap,
}

impl CommonMapBundle {
    pub fn from_handle(map_data: Handle<MapData>) -> Self {
        Self {
            collider: default(),
            transform: Transform::from_scale(Vec3::splat(MAP_SCALE)),
            global_transform: default(),
            map_data,
            current_map: default(),
        }
    }
}

fn setup_walk_collisions_system(
    mut current_map_query: Query<(&mut Collider, &Handle<MapData>), With<CurrentMap>>,
    maps: Res<Assets<MapData>>,
) {
    if let Ok((mut map_collider, map_data_handle)) = current_map_query.get_single_mut() {
        if let Some(map_data) = maps.get(map_data_handle) {
            *map_collider = Collider::trimesh_with_flags(
                map_data.walk_mask.vertices.clone(),
                map_data.walk_mask.indices.clone(),
                TriMeshFlags::MERGE_DUPLICATE_VERTICES,
            );
        }
    }
}

fn leave_construction_system(mut load_state: ResMut<NextState<MapLoadState>>) {
    load_state.set(MapLoadState::Loaded);
}

/// When the map data asset is finished loading the state is changed to construction
fn map_check_load_system(
    current_map_query: Query<&Handle<MapData>, With<CurrentMap>>,
    maps: Res<Assets<MapData>>,
    mut load_state: ResMut<NextState<MapLoadState>>,
) {
    if let Ok(map_handle) = current_map_query.get_single() {
        if maps.get(map_handle).is_some() {
            load_state.set(MapLoadState::Constructing);
        }
    }
}

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum MapLoadState {
    #[default]
    Unloaded,
    Loading,
    Constructing,
    Loaded,
}

pub struct CommonMapPlugin;

impl Plugin for CommonMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<MapData>()
            .init_asset_loader::<MapDataLoader>()
            .init_state::<MapLoadState>()
            .add_systems(
                FixedUpdate,
                map_check_load_system.run_if(in_state(MapLoadState::Loading)),
            )
            .add_systems(
                OnEnter(MapLoadState::Constructing),
                (setup_walk_collisions_system, leave_construction_system),
            );
    }
}
