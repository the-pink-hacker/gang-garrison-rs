use bevy::prelude::*;

use crate::entity::entities::MapEntity;

pub mod io;

#[derive(Debug, Asset, TypePath)]
pub struct MapData {
    pub entities: Vec<MapEntity>,
    pub walk_mask: Vec<()>,
}
