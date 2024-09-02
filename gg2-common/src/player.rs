use bevy::{ecs::system::EntityCommands, prelude::*};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    error::{Error, Result},
    networking::message::ServerPlayerJoin,
};

#[derive(Debug, Default, Resource)]
pub struct Players(Vec<Entity>);

impl Players {
    pub fn get_entity(&self, player_index: u8) -> Option<&Entity> {
        self.0.get(player_index as usize)
    }

    pub fn add_player<'a>(
        &mut self,
        commands: &'a mut Commands,
        player_component: Player,
    ) -> EntityCommands<'a> {
        let player = commands.spawn(player_component);
        self.0.push(player.id());
        player
    }

    pub fn remove_player(&mut self, commands: &mut Commands, player_index: u8) -> Result<()> {
        let entity = if (player_index as usize) < self.0.len() {
            Ok(self.0.remove(player_index as usize))
        } else {
            Err(Error::PlayerLookup(player_index))
        }?;

        commands
            .get_entity(entity)
            .ok_or(Error::PlayerLookup(player_index))?
            .despawn_recursive();

        Ok(())
    }
}

#[derive(Debug, Component)]
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

#[derive(Debug, Default, Component, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Team {
    Red,
    Blu,
    #[default]
    Spectator,
    Any,
}

#[derive(Debug, Default, Component, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Class {
    #[default]
    Scout,
    Soldier,
    Sniper,
    Demoman,
    Medic,
    Engineer,
    Heavy,
    Spy,
    Pyro,
    Quote,
}
