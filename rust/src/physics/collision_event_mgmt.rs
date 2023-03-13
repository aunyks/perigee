use crossbeam::channel::{Receiver, TryRecvError};
use rapier3d::prelude::{ChannelEventCollector, CollisionEvent, ContactForceEvent};

use crate::config::PhysicsConfig;

/// A structure for managing contact events (collision and contact force events)
/// that took place during physics simulation.
pub struct ContactEventManager {
    channel_event_collector: ChannelEventCollector,
    collision_event_receiver: Receiver<CollisionEvent>,
    contact_force_event_receiver: Receiver<ContactForceEvent>,
}

impl Default for ContactEventManager {
    fn default() -> Self {
        let default_physics_config = PhysicsConfig::default();
        Self::with_capacity(default_physics_config.event_queue_capacity())
    }
}

impl ContactEventManager {
    pub fn with_capacity(cap: usize) -> Self {
        let (collision_send, collision_recv) = crossbeam::channel::bounded(cap);
        let (contact_force_send, contact_force_recv) = crossbeam::channel::bounded(cap);
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        Self {
            channel_event_collector: event_handler,
            collision_event_receiver: collision_recv,
            contact_force_event_receiver: contact_force_recv,
        }
    }

    pub fn event_collector(&self) -> &ChannelEventCollector {
        &self.channel_event_collector
    }

    pub fn get_collider_event(&self) -> Result<CollisionEvent, TryRecvError> {
        self.collision_event_receiver.try_recv()
    }

    pub fn get_contact_force_event(&self) -> Result<ContactForceEvent, TryRecvError> {
        self.contact_force_event_receiver.try_recv()
    }

    pub fn eviscerate_channels(&self) -> Result<(), TryRecvError> {
        while !self.contact_force_event_receiver.is_empty() {
            match self.get_contact_force_event() {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        while !self.collision_event_receiver.is_empty() {
            match self.get_collider_event() {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        Ok(())
    }
}
