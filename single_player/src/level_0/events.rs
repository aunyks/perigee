use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Level0Event {
    LevelCompleted,
    LevelFailed,
}

impl From<Level0Event> for i32 {
    fn from(level_event: Level0Event) -> Self {
        match level_event {
            Level0Event::LevelCompleted => 0,
            Level0Event::LevelFailed => 1,
        }
    }
}
