use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerEvent {
    /// Player just jumped
    Jump,
    /// Player just landed on a surface
    Landed,
    Moving,
    Stopped,
    Crouched,
    StoodUpright,
    /// Player just took a footstep
    Stepped,
    StartedWallRunning,
    StoppedWallRunning,
    /// Player started sliding on the ground
    StartedSliding,
    StoppedSliding,
}

impl From<PlayerEvent> for u32 {
    fn from(event: PlayerEvent) -> Self {
        match event {
            PlayerEvent::Jump => 0,
            PlayerEvent::Landed => 1,
            PlayerEvent::Moving => 2,
            PlayerEvent::Stopped => 3,
            PlayerEvent::Crouched => 4,
            PlayerEvent::StoodUpright => 5,
            PlayerEvent::Stepped => 6,
            PlayerEvent::StartedWallRunning => 7,
            PlayerEvent::StoppedWallRunning => 8,
            PlayerEvent::StartedSliding => 9,
            PlayerEvent::StoppedSliding => 10,
        }
    }
}
