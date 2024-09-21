use bevy::prelude::*;
use collision::mesh::WalkMeshMask;

use crate::entity::entities::MapEntity;

pub mod collision;
pub mod io;

#[derive(Debug, Asset, TypePath)]
pub struct MapData {
    pub entities: Vec<MapEntity>,
    pub walk_mask: WalkMeshMask,
}
