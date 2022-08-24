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
