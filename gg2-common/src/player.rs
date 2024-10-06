use std::fmt::Display;

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    error::{Error, Result},
    game::InGameOnly,
    networking::message::ServerPlayerJoin,
};

pub mod class;
pub mod team;

#[derive(Component)]
struct MarkedForRemoval;

#[derive(Debug, Default, Resource)]
pub struct Players(Vec<Entity>);

impl Players {
    pub fn add_player<'a, T: Bundle>(
        &mut self,
        commands: &'a mut Commands,
        player: T,
    ) -> EntityCommands<'a> {
        let player = commands.spawn((player, PlayerId::from(self.0.len() as u8), InGameOnly));
        self.0.push(player.id());
        player
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn get_entity(&self, player_index: impl Into<PlayerId>) -> Result<Entity> {
        let player_index = player_index.into();
        self.0
            .get(usize::from(player_index))
            .cloned()
            .ok_or(Error::PlayerLookup(player_index))
    }

    pub fn mark_remove(
        &self,
        commands: &mut Commands,
        player_index: impl Into<PlayerId>,
    ) -> Result<()> {
        let player_index = player_index.into();
        let entity = self.get_entity(player_index)?;

        commands
            .get_entity(entity)
            .ok_or(Error::PlayerLookup(player_index))?
            .insert(MarkedForRemoval);

        Ok(())
    }
}

impl<'a> IntoIterator for &'a Players {
    type Item = &'a Entity;
    type IntoIter = std::slice::Iter<'a, Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Debug, Component, Default)]
pub struct Player {
    pub name: String,
}

impl From<ServerPlayerJoin> for Player {
    fn from(value: ServerPlayerJoin) -> Self {
        Self {
            name: value.player_name,
        }
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct PositionShift(pub Vec2);

impl From<Vec2> for PositionShift {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

#[derive(Debug, Component, Clone, Copy)]
pub struct PlayerId(u8);

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        u8::from(*self).fmt(f)
    }
}

impl From<PlayerId> for u8 {
    fn from(value: PlayerId) -> Self {
        value.0
    }
}

impl From<PlayerId> for usize {
    fn from(value: PlayerId) -> Self {
        value.0 as usize
    }
}

impl From<u8> for PlayerId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct RawInput {
    // TODO: Add key state
    pub key_state: u8,
    pub net_aim_direction: u16,
    pub aim_distance: f32,
}

#[derive(Debug, Clone)]
pub struct RawPlayerInfo {
    pub position: Vec2,
    pub velocity: Vec2,
    pub health: u8,
    pub ammo_count: u8,
    // TODO: Add move status
    pub move_status: u8,
}

#[derive(Debug, Clone)]
pub struct RawAdditionalPlayerInfo {}

fn remove_stale_players_system(
    mut commands: Commands,
    query: Query<(Entity, &PlayerId), With<MarkedForRemoval>>,
    mut players: ResMut<Players>,
) {
    let mut remove_indices = Vec::new();

    for (entity, player_index) in query.iter() {
        commands.entity(entity).despawn();

        remove_indices.push(u8::from(*player_index));
    }

    // Sorted in reverse to prevent index shifting.
    remove_indices.sort_by(|a, b| b.cmp(a));

    for index in remove_indices {
        if (index as usize) < players.0.len() {
            players.0.remove(index.into());
        } else {
            eprintln!("Failed to remove player of index: {}", index);
        }
    }
}

pub struct CommonPlayerPlugin;

impl Plugin for CommonPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, remove_stale_players_system);
    }
}
