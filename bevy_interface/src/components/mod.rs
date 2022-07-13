use bevy::ecs::component::Component;

/// This component is used to define an entity that exists in one level at a time. It should
/// be set up and torn down with every level transition.
#[derive(Component)]
pub struct LevelObject;
