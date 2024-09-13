use bevy::prelude::*;

pub mod io;

#[derive(Debug, Asset, TypePath)]
pub struct MapData {
    entities: Vec<()>,
    wall_mask: Vec<()>,
}
