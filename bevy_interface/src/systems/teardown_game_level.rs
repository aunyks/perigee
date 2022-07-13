use crate::components::LevelObject;
use bevy::prelude::*;

/// Removes every entity with a [`LevelObject`](crate::components::LevelObject) component from the level
pub fn teardown_game_level(mut commands: Commands, query: Query<Entity, With<LevelObject>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
