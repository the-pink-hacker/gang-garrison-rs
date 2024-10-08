use bevy::prelude::*;
use bevy_rapier2d::prelude::{ColliderDisabled, RigidBodyDisabled};
use gg2_common::{
    error::Error,
    networking::message::{
        ServerPlayerChangeClass, ServerPlayerChangeTeam, ServerPlayerJoin, ServerPlayerLeave,
        ServerQuickUpdate,
    },
    player::{class::ClassGeneric, team::Team, CommonPlayerPlugin, Player, Players, PositionShift},
};

use crate::{
    networking::{state::NetworkingState, NetworkData},
    state::ClientState,
};

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
}

fn handle_player_join_system(
    mut events: EventReader<NetworkData<ServerPlayerJoin>>,
    mut commands: Commands,
    mut players: ResMut<Players>,
) {
    for event in events.read() {
        println!("Player join of name: \"{}\"", event.player_name);

        players.add_player(
            &mut commands,
            PlayerBundle {
                player: (*event).clone().into(),
                sprite: SpriteBundle {
                    visibility: Visibility::Hidden,
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Center,
                        ..default()
                    },
                    ..default()
                },
            },
        );
    }
}

fn handle_player_change_team_system(
    mut events: EventReader<NetworkData<ServerPlayerChangeTeam>>,
    mut commands: Commands,
    players: Res<Players>,
) {
    for event in events.read() {
        let player_result = players
            .get_entity(event.player_index)
            .and_then(|player| {
                commands
                    .get_entity(player)
                    .ok_or(Error::PlayerLookup(event.player_index))
            })
            .map(|mut player| {
                println!(
                    "Player of index {} is changing teams to: {:?}",
                    event.player_index, event.player_team
                );
                player.insert(event.player_team);

                match event.player_team {
                    Team::Red | Team::Blu => {
                        player
                            .insert(Visibility::Visible)
                            .remove::<(RigidBodyDisabled, ColliderDisabled)>();
                    }
                    Team::Spectator => {
                        player.insert((RigidBodyDisabled, ColliderDisabled, Visibility::Hidden));
                    }
                }
            });

        if let Err(error) = player_result {
            eprintln!("Failed to change player's team: {}", error);
        }
    }
}

fn handle_player_change_class_system(
    mut events: EventReader<NetworkData<ServerPlayerChangeClass>>,
    mut commands: Commands,
    players: Res<Players>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let player_result = players
            .get_entity(event.player_index)
            .and_then(|player| {
                commands
                    .get_entity(player)
                    .ok_or(Error::PlayerLookup(event.player_index))
            })
            .map(|mut player| {
                println!(
                    "Player of index {} is changing class to: {:?}",
                    event.player_index, event.player_class
                );
                let player_texture = asset_server.load::<Image>("sprites/character.png");
                event.player_class.add_class_components(&mut player);
                player.insert(player_texture);
            });

        if let Err(error) = player_result {
            eprintln!("Failed to change player's class: {}", error);
        }
    }
}

fn handle_quick_update_system(
    mut events: EventReader<NetworkData<ServerQuickUpdate>>,
    players: Res<Players>,
    mut player_query: Query<(&mut Transform, &PositionShift), With<Player>>,
) {
    for event in events.read() {
        for (player_index, player) in players.as_ref().into_iter().enumerate() {
            let player_result = player_query
                .get_mut(*player)
                .map_err(|_| Error::PlayerLookup((player_index as u8).into()))
                .map(|(mut transform, position_shift)| {
                    if let Some((_input, player_info)) = event
                        .player_characters
                        .get(player_index)
                        .and_then(Option::as_ref)
                    {
                        let new_position = player_info.position + **position_shift;
                        transform.translation = new_position.extend(10.0);
                    }
                });

            if let Err(error) = player_result {
                eprintln!("Failed to update player: {}", error);
            }
        }
    }
}

fn handle_player_leave_system(
    mut commands: Commands,
    mut events: EventReader<NetworkData<ServerPlayerLeave>>,
    players: Res<Players>,
) {
    for event in events.read() {
        println!("Leaving: {}", event.player_index);
        if let Err(error) = players.mark_remove(&mut commands, event.player_index) {
            eprintln!("Failed to mark player for removal: {}", error);
        }
    }
}

fn clear_players(mut players: ResMut<Players>) {
    players.clear();
}

fn debug_players_system(player_query: Query<(&Player, &ClassGeneric, &Team)>) {
    player_query.iter().for_each(|(player, class, team)| {
        debug!(
            "[Player] name: \"{}\", class: {:?}, team: {:?}",
            player.name, class, team
        );
    });
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Players>()
            .add_plugins(CommonPlayerPlugin)
            .add_systems(
                FixedUpdate,
                (
                    handle_player_join_system,
                    (
                        handle_player_change_team_system,
                        handle_player_change_class_system,
                        handle_quick_update_system,
                        debug_players_system,
                        handle_player_leave_system,
                    )
                        .run_if(
                            in_state(NetworkingState::InGame)
                                .or_else(in_state(NetworkingState::PlayerJoining)),
                        )
                        .after(handle_player_join_system),
                ),
            )
            .add_systems(OnExit(ClientState::InGame), clear_players);
    }
}
