use serde::{Deserialize, Serialize};

/// Configuration parameters for the [PhysicsWorld](crate::physics::PhysicsWorld).
/// These should not be editable at runtime.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PhysicsConfig {
    gravity: [f32; 3],
    event_queue_capacity: usize,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: [0.0, -9.81, 0.0],
            event_queue_capacity: 5,
        }
    }
}

impl PhysicsConfig {
    pub fn gravity(&self) -> [f32; 3] {
        self.gravity
    }

    pub fn event_queue_capacity(&self) -> usize {
        self.event_queue_capacity
    }
}
