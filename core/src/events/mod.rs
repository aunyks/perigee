pub use audiovisual::AudioVisualEvent;
pub use player::PlayerEvent;
use serde::{Deserialize, Serialize};

pub mod audiovisual;
mod player;

const EVENT_TYPE_OFFSET: u32 = u32::MAX / 3;

/// An event that can take place during simulation
/// of the level. The two core event types
/// (`Player` and `AudioVisual`) can occur in any level.
///  
/// The `LevelEvent(T)` is specific to the current level
/// being simulated where `T` is the level-specific event type.
///
/// Note: u32 must implement `From<T>`!
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum GameEvent<T> {
    Player(PlayerEvent),
    AudioVisual(AudioVisualEvent),
    LevelEvent(T),
}

impl<T> Default for GameEvent<T> {
    fn default() -> Self {
        GameEvent::Player(PlayerEvent::Stopped)
    }
}

impl<T> From<PlayerEvent> for GameEvent<T> {
    fn from(player_event: PlayerEvent) -> Self {
        GameEvent::Player(player_event)
    }
}

impl<T> From<AudioVisualEvent> for GameEvent<T> {
    fn from(audiovisual_event: AudioVisualEvent) -> Self {
        GameEvent::AudioVisual(audiovisual_event)
    }
}

impl<T> From<GameEvent<T>> for u32
where
    u32: From<T>,
{
    fn from(event: GameEvent<T>) -> Self {
        match event {
            #[allow(clippy::erasing_op)]
            GameEvent::Player(player_event) => {
                let player_event_id: u32 = player_event.into();
                (EVENT_TYPE_OFFSET * 0) + player_event_id
            }
            #[allow(clippy::identity_op)]
            GameEvent::AudioVisual(audiovisual_event) => {
                let av_event_id: u32 = audiovisual_event.into();
                (EVENT_TYPE_OFFSET * 1) + av_event_id
            }
            GameEvent::LevelEvent(level_event) => {
                let level_event_id: u32 = level_event.into();
                (EVENT_TYPE_OFFSET * 2) + level_event_id
            }
        }
    }
}
