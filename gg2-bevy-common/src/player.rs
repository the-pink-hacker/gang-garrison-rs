use bevy::{
    ecs::{
        query::{QueryData, WorldQuery},
        system::EntityCommands,
    },
    prelude::*,
};
use gg2_common::{
    error::*,
    player::{KeyState, PlayerId, class::ClassGeneric, team::Team},
};

use crate::game::InGameOnly;

pub mod character;

#[derive(Component)]
struct MarkedForRemoval;

#[derive(Debug, Default, Resource)]
pub struct Players(Vec<Entity>);

impl Players {
    pub fn add_player<'a>(
        &mut self,
        commands: &'a mut Commands,
        player: impl Bundle,
    ) -> Result<(PlayerId, EntityCommands<'a>)> {
        let player = commands.spawn((player, Team::default(), ClassGeneric::default(), InGameOnly));

        //ClassGeneric::default().add_class_components(&mut player);

        let new_player_id = self.get_next_id().ok_or(Error::PlayerIdTooMany)?;
        self.0.push(player.id());
        Ok((new_player_id, player))
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

    pub fn query_entity<'q, 'w, 's, D>(
        &self,
        player_index: impl Into<PlayerId>,
        query: &'q Query<'w, 's, D>,
    ) -> Result<(Entity, <D as WorldQuery>::Item<'q>)>
    where
        D: QueryData<ReadOnly = D>,
    {
        let player_index = player_index.into();
        let player_entity = self.get_entity(player_index)?;

        query
            .get(player_entity)
            .map_err(|_| Error::PlayerLookup(player_index))
            .map(|query_result| (player_entity, query_result))
    }

    pub fn query_mut_entity<'q, 'w, 's, D>(
        &self,
        player_index: impl Into<PlayerId>,
        query: &'q mut Query<'w, 's, D>,
    ) -> Result<(Entity, <D as WorldQuery>::Item<'q>)>
    where
        D: QueryData,
    {
        let player_index = player_index.into();
        let player_entity = self.get_entity(player_index)?;

        query
            .get_mut(player_entity)
            .map_err(|_| Error::PlayerLookup(player_index))
            .map(move |query_result| (player_entity, query_result))
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

    pub fn len(&self) -> u8 {
        self.0.len().try_into().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_id(&self, player: Entity) -> Option<PlayerId> {
        self.0.iter().enumerate().find_map(|(index, entity)| {
            if *entity == player {
                PlayerId::try_from(index).ok()
            } else {
                None
            }
        })
    }

    pub fn get_next_id(&self) -> Option<PlayerId> {
        self.len().try_into().ok()
    }
}

impl<'a> IntoIterator for &'a Players {
    type Item = &'a Entity;
    type IntoIter = std::slice::Iter<'a, Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct PositionShift(pub Vec2);

impl From<Vec2> for PositionShift {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

fn remove_stale_players_system(
    mut commands: Commands,
    query: Query<Entity, With<MarkedForRemoval>>,
    mut players: ResMut<Players>,
) {
    let mut remove_indices = Vec::new();

    for entity in query.iter() {
        if let Some(player_index) = players.get_id(entity) {
            commands.entity(entity).despawn();

            remove_indices.push(player_index);
        }
    }

    // Sorted in reverse to prevent index shifting.
    remove_indices.sort_by(|a, b| b.cmp(a));

    for index in remove_indices {
        if usize::from(index) < players.0.len() {
            players.0.remove(index.into());
        } else {
            error!("Failed to remove player of index: {}", index);
        }
    }
}

pub struct CommonPlayerPlugin;

impl Plugin for CommonPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, remove_stale_players_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_to_key_state_up() {
        let raw = 0b1000_0000u8;

        assert_eq!(
            KeyState::from(raw),
            KeyState {
                up: true,
                ..default()
            }
        );
    }

    #[test]
    fn u8_to_key_state_down() {
        let raw = 0b0000_0010u8;

        assert_eq!(
            KeyState::from(raw),
            KeyState {
                down: true,
                ..default()
            }
        );
    }

    #[test]
    fn u8_to_key_state_left() {
        let raw = 0b0100_0000u8;

        assert_eq!(
            KeyState::from(raw),
            KeyState {
                left: true,
                ..default()
            }
        );
    }

    #[test]
    fn u8_to_key_state_right() {
        let raw = 0b0010_0000u8;

        assert_eq!(
            KeyState::from(raw),
            KeyState {
                right: true,
                ..default()
            }
        );
    }

    #[test]
    fn u8_to_key_state_multiple() {
        let raw = 0b1110_0000u8;

        assert_eq!(
            KeyState::from(raw),
            KeyState {
                up: true,
                down: false,
                left: true,
                right: true,
            }
        );
    }

    #[test]
    fn key_state_to_u8_up() {
        assert_eq!(
            u8::from(KeyState {
                up: true,
                ..default()
            }),
            0b1000_0000u8
        );
    }

    #[test]
    fn key_state_to_u8_down() {
        assert_eq!(
            u8::from(KeyState {
                down: true,
                ..default()
            }),
            0b0000_0010u8
        );
    }

    #[test]
    fn key_state_to_u8_left() {
        assert_eq!(
            u8::from(KeyState {
                left: true,
                ..default()
            }),
            0b0100_0000u8
        );
    }

    #[test]
    fn key_state_to_u8_right() {
        assert_eq!(
            u8::from(KeyState {
                right: true,
                ..default()
            }),
            0b0010_0000u8
        );
    }

    #[test]
    fn key_state_to_u8_multiple() {
        assert_eq!(
            u8::from(KeyState {
                up: true,
                down: true,
                left: false,
                right: true,
            }),
            0b1010_0010u8
        );
    }
}
