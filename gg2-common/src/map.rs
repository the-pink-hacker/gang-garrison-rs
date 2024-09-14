use bevy::prelude::*;
use serde::Deserialize;

use crate::entity::entities::MapEntity;

pub mod io;

#[derive(Debug, Deserialize)]
pub struct MapDataEntities(Vec<MapEntity>);

#[derive(Debug, Asset, TypePath)]
pub struct MapData {
    entities: MapDataEntities,
    wall_mask: Vec<()>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "[{background:ffffff,type:meta,void:000000},{type:spawnroom,x:924,xscale:6,y:498,yscale:3},{type:spawnroom,x:3684,xscale:6,y:498,yscale:3},{type:medCabinet,x:1026,xscale:1.31,y:558,yscale:1.38},{type:medCabinet,x:3792,xscale:1.31,y:558,yscale:1.38},{type:redteamgate,x:906,xscale:3,y:546,yscale:1.30},{type:blueteamgate,x:3936,xscale:3,y:546,yscale:1.30},{type:redteamgate,x:1176,xscale:3,y:546,yscale:1.30},{type:blueteamgate,x:3666,xscale:3,y:546,yscale:1.30},{type:redspawn,x:1002,y:598},{type:bluespawn,x:3854,y:598},{type:redspawn,x:1044,y:598},{type:bluespawn,x:3812,y:598},{type:redspawn,x:1086,y:598},{type:bluespawn,x:3770,y:598},{type:redintel,x:627,y:822},{type:blueintel,x:4231,y:822}]";

    #[test]
    fn debug() {
        let data = DATA.replace(',', "\n").replace('}', "\n}");
        let x = serde_hjson::from_str::<MapDataEntities>(&data).unwrap();
        dbg!(x);
    }
}
