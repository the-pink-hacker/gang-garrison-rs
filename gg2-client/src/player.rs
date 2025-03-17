use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::{ColliderDisabled, RigidBodyDisabled};
use gg2_common::{
    error::Error,
    map::{CurrentMap, MapData},
    networking::message::*,
    player::{
        class::ClassGeneric, team::Team, CommonPlayerPlugin, Player, PlayerId, Players,
        PositionShift,
    },
};

use crate::{
    config::ClientConfig,
    networking::{state::NetworkingState, NetworkData},
    state::ClientState,
};

#[derive(Component)]
pub struct ClientPlayer;

/// Stores the current player id until the client player joins.
#[derive(Resource, Deref)]
pub struct ClientPlayerAssign(PlayerId);

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
        debug!("Player join of name: \"{}\"", event.player_name);

        add_player(&mut commands, &mut players, (*event).clone());
    }
}

fn add_client_player_system(
    client_player_assign: Res<ClientPlayerAssign>,
    mut players: ResMut<Players>,
    mut commands: Commands,
    config: Res<ClientConfig>,
) {
    let client_player_id = **client_player_assign.as_ref();
    let next_player_id = players.get_next_id();

    if client_player_id == next_player_id {
        debug!("Marking client player of id: {}", client_player_id);
        let (_, mut player) = add_player(
            &mut commands,
            &mut players,
            Player {
                name: config.game.player_name.clone(),
            },
        );
        player.insert(ClientPlayer);
        commands.remove_resource::<ClientPlayerAssign>();
    }
}

fn add_player<'a>(
    commands: &'a mut Commands,
    players: &mut Players,
    player: impl Into<Player>,
) -> (PlayerId, EntityCommands<'a>) {
    players.add_player(
        commands,
        PlayerBundle {
            player: player.into(),
            sprite: SpriteBundle {
                visibility: Visibility::Hidden,
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
                ..default()
            },
        },
    )
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
                debug!(
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
            error!("Failed to change player's team: {}", error);
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
                debug!(
                    "Player of index {} is changing class to: {:?}",
                    event.player_index, event.player_class
                );
                let player_texture = asset_server.load::<Image>("sprites/character.png");
                event.player_class.add_class_components(&mut player);
                player.insert(player_texture);
            });

        if let Err(error) = player_result {
            error!("Failed to change player's class: {}", error);
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
                error!("Failed to update player: {}", error);
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
        debug!("Player is leaving of index: {}", event.player_index);
        if let Err(error) = players.mark_remove(&mut commands, event.player_index) {
            error!("Failed to mark player for removal: {}", error);
        }
    }
}

fn clear_players(mut players: ResMut<Players>) {
    players.clear();
}

#[allow(unused)]
fn debug_players_system(player_query: Query<(&Player, &ClassGeneric, &Team)>) {
    player_query.iter().for_each(|(player, class, team)| {
        trace!(
            "[Player] name: \"{}\", class: {:?}, team: {:?}",
            player.name,
            class,
            team
        );
    });
}

fn listen_for_client_player_id_system(
    mut events: EventReader<NetworkData<ServerJoinUpdate>>,
    mut commands: Commands,
) {
    for event in events.read() {
        debug!(
            "Waiting for client player to join of id: {}",
            event.client_player_id
        );
        commands.insert_resource(ClientPlayerAssign(event.client_player_id));
    }
}

fn handle_player_spawn(
    mut events: EventReader<NetworkData<ServerPlayerSpawn>>,
    map_data_query: Query<&Handle<MapData>, With<CurrentMap>>,
    map_data_assets: Res<Assets<MapData>>,
    players: Res<Players>,
    mut player_query: Query<(&mut Transform, &Team)>,
) {
    for event in events.read() {
        debug!("{:#?}", **event);

        if let Some((mut player_transform, player_team)) = players
            .get_entity(event.player_index)
            .ok()
            .and_then(|entity| player_query.get_mut(entity).ok())
        {
            if let Some(map_data) = map_data_query
                .get_single()
                .ok()
                .and_then(|handle| map_data_assets.get(handle))
            {
                match player_team
                    .try_into()
                    .and_then(|team| map_data.get_spawn_position(&team, event.spawn_index))
                {
                    Ok(spawn_position) => {
                        player_transform.translation.x = spawn_position.x;
                        player_transform.translation.y = spawn_position.y;
                        debug!(
                            "Spawned player {} at {}, {}",
                            event.player_index, spawn_position.x, spawn_position.y
                        );
                    }
                    Err(error) => error!("Failed to spawn player: {}", error),
                }
            } else {
                error!("Failed to query map data.");
            }
        } else {
            error!("Failed to find player of id: {}", event.player_index);
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Players>()
            .add_plugins(CommonPlayerPlugin)
            .add_systems(
                FixedUpdate,
                (
                    listen_for_client_player_id_system
                        .before(add_client_player_system)
                        .run_if(in_state(NetworkingState::PlayerJoining)),
                    add_client_player_system
                        .before(handle_player_join_system)
                        .run_if(resource_exists::<ClientPlayerAssign>),
                    handle_player_join_system,
                    (
                        handle_player_change_team_system,
                        handle_player_change_class_system,
                        handle_quick_update_system,
                        //debug_players_system,
                        handle_player_leave_system,
                        handle_player_spawn,
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
