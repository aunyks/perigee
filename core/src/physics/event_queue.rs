use crate::data_structures::Queue;
use rapier3d::pipeline::EventHandler as RapierEventHandler;
use rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
pub struct CollisionEventQueue {
    // This Arc + Mutex wrapper is a workaround
    // for `handle_collision_event()` not mutably
    // borrowing `self`. If it mutably borrows
    // `self` in the future, please remove this.
    // It looks unnecessarily ugly
    queue: Arc<Mutex<Queue<CollisionEvent>>>,
}

impl CollisionEventQueue {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(Queue::with_capacity(cap))),
        }
    }

    fn enqueue(&self, event: CollisionEvent) {
        self.queue.lock().unwrap().enqueue(event);
    }

    pub fn dequeue(&self) -> CollisionEvent {
        self.queue.lock().unwrap().dequeue()
    }

    pub fn peek(&self) -> Option<CollisionEvent> {
        self.queue.lock().unwrap().peek().cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.queue.lock().unwrap().capacity()
    }

    pub fn size(&self) -> usize {
        self.queue.lock().unwrap().size()
    }
}

impl RapierEventHandler for CollisionEventQueue {
    fn handle_collision_event(
        &self,
        _bodies: &RigidBodySet,
        _colliders: &ColliderSet,
        event: CollisionEvent,
        _contact_pair: Option<&ContactPair>,
    ) {
        self.enqueue(event);
    }

    fn handle_contact_force_event(
        &self,
        _dt: Real,
        _bodies: &RigidBodySet,
        _colliders: &ColliderSet,
        _contact_pair: &ContactPair,
        _total_force_magnitude: Real,
    ) {
    }
}
