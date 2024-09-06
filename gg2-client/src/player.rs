use bevy::prelude::*;
use gg2_common::{
    error::Error,
    networking::message::{ServerPlayerChangeClass, ServerPlayerChangeTeam, ServerPlayerJoin},
    player::{Class, Player, Players, Team},
};

use crate::networking::{state::NetworkingState, NetworkData};

fn handle_player_join(
    mut events: EventReader<NetworkData<ServerPlayerJoin>>,
    mut commands: Commands,
    mut players: ResMut<Players>,
) {
    for event in events.read() {
        println!("Player join of name: \"{}\"", event.player_name);
        let player = (**event).clone().into();
        players
            .add_player(&mut commands, player)
            .insert((Team::default(), Class::default()));
    }
}

fn handle_player_change_team(
    mut events: EventReader<NetworkData<ServerPlayerChangeTeam>>,
    mut commands: Commands,
    players: Res<Players>,
) {
    for event in events.read() {
        let player_result = players
            .get_entity(event.player_index)
            .ok_or(Error::PlayerLookup(event.player_index))
            .and_then(|player| {
                commands
                    .get_entity(*player)
                    .ok_or(Error::PlayerLookup(event.player_index))
            })
            .map(|mut player| {
                println!(
                    "Player of index {} is changing teams to: {:?}",
                    event.player_index, event.player_team
                );
                player.insert(event.player_team);
            });

        if let Err(error) = player_result {
            eprintln!("Failed to change player's team: {}", error);
        }
    }
}

fn handle_player_change_class(
    mut events: EventReader<NetworkData<ServerPlayerChangeClass>>,
    mut commands: Commands,
    players: Res<Players>,
) {
    for event in events.read() {
        let player_result = players
            .get_entity(event.player_index)
            .ok_or(Error::PlayerLookup(event.player_index))
            .and_then(|player| {
                commands
                    .get_entity(*player)
                    .ok_or(Error::PlayerLookup(event.player_index))
            })
            .map(|mut player| {
                println!(
                    "Player of index {} is changing class to: {:?}",
                    event.player_index, event.player_class
                );
                player.insert(event.player_class);
            });

        if let Err(error) = player_result {
            eprintln!("Failed to change player's class: {}", error);
        }
    }
}

fn debug_players(player_query: Query<(&Player, &Class, &Team)>) {
    player_query.iter().for_each(|(player, class, team)| {
        debug!(
            "[Player] name: \"{}\", class: {:?}, team: {:?}",
            player.name, class, team
        )
    });
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Players>().add_systems(
            FixedUpdate,
            (
                handle_player_join.run_if(in_state(NetworkingState::PlayerJoining)),
                (
                    handle_player_change_team,
                    handle_player_change_class,
                    debug_players,
                )
                    .run_if(in_state(NetworkingState::PlayerJoining))
                    .run_if(in_state(NetworkingState::InGame))
                    .after(handle_player_join),
            ),
        );
    }
}
