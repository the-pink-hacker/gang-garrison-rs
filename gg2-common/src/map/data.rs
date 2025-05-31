use glam::Vec2;

use crate::{error::CommonError, gamemode::Gamemode, player::team::TeamSpawnable};

#[derive(Debug)]
pub struct MapData {
    pub walk_mask: Vec<u8>,
    /// A slice of all blu spawn groups with list of spawn positions
    pub blu_spawns: [Vec<Vec2>; 5],
    /// A slice of all red spawn groups with list of spawn positions
    pub red_spawns: [Vec<Vec2>; 5],
    pub gamemode: Gamemode,
}

impl MapData {
    pub fn get_spawn_position(
        &self,
        team: &TeamSpawnable,
        spawn_group: u8,
        index: u8,
    ) -> Result<&Vec2, CommonError> {
        match team {
            TeamSpawnable::Blu => &self.blu_spawns,
            TeamSpawnable::Red => &self.red_spawns,
        }
        .get(spawn_group as usize)
        .ok_or(CommonError::SpawnLookup(*team, spawn_group, index))?
        .get(index as usize)
        .ok_or(CommonError::SpawnLookup(*team, spawn_group, index))
    }
}
