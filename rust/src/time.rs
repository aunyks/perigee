use std::cmp::{PartialEq, PartialOrd};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// A clock that passes time whenever it's
/// ticked. This clock is great for pure simulation
/// timekeeping as it doesn't need access to system-native
/// clocks and only ticks through the provided `tick()` function.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PassiveClock(Duration);

impl PassiveClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_seconds(&mut self, seconds: f32) {
        self.0 = Duration::from_secs_f32(seconds);
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        self.0 = self
            .0
            .saturating_add(Duration::from_secs_f32(delta_seconds));
    }

    pub fn tick_reverse(&mut self, delta_seconds: f32) {
        self.0 = self
            .0
            .saturating_sub(Duration::from_secs_f32(delta_seconds));
    }

    pub fn reset(&mut self) {
        self.0 = Duration::default();
    }

    pub fn elapsed(&self) -> Duration {
        self.0
    }
}
