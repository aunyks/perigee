#![allow(dead_code)]

/// This enum defines settings for the [`FirstPersonControlPlugin`](crate::plugins::FirstPersonControlPlugin).
/// `Enabled` causes the plugin to listen for keyboard
/// and mouse events. `Disabled` causes the plugin to ignore
/// keyboard and mouse events.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FirstPersonControlSettings {
    Disabled,
    Enabled,
}

/// This enum defines the active level. A level is a screen context, so
/// a main menu, gameplay level, cinematic sequence, and more are all levels
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameLevel {
    Zero,
    PauseMenu,
}
