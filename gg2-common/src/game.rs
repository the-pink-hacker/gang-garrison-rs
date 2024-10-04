use bevy::prelude::*;

#[derive(Component, Default)]
pub struct InGameOnly;

pub fn garbage_collect_in_game_only_system(
    mut commands: Commands,
    query: Query<Entity, With<InGameOnly>>,
) {
    println!("Despawning in-game only entities.");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
