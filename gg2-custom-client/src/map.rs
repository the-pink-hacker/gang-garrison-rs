use crate::prelude::*;

#[derive(Default)]
pub struct MapInfo {
    pub current_map: Option<(AssetId, MapData)>,
}
