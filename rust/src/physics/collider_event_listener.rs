use crate::event_channel::EventChannel;
use crate::traits::physics::ColliderEventListener;
use crossbeam::channel::Sender;
use rapier3d::prelude::*;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum ColliderEvent {
    CollisionStart(ColliderHandle),
    CollisionEnd(ColliderHandle),
    IntersectionStart(ColliderHandle),
    IntersectionEnd(ColliderHandle),
    ContactForceEvent(ColliderHandle, ContactForceEvent),
}

#[derive(Default)]
pub struct ColliderEventChannel {
    inner: EventChannel<ColliderEvent>,
}

impl ColliderEventChannel {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: EventChannel::with_capacity(cap),
        }
    }
}

impl Deref for ColliderEventChannel {
    type Target = EventChannel<ColliderEvent>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct ColliderEventRelayer {
    inner: Sender<ColliderEvent>,
}

impl From<Sender<ColliderEvent>> for ColliderEventRelayer {
    fn from(sender: Sender<ColliderEvent>) -> Self {
        Self { inner: sender }
    }
}

impl ColliderEventListener for ColliderEventRelayer {
    fn on_collision_start(&mut self, other: &ColliderHandle) {
        let _send_result = self.inner.send(ColliderEvent::CollisionStart(*other));
    }

    fn on_collision_end(&mut self, other: &ColliderHandle) {
        let _send_result = self.inner.send(ColliderEvent::CollisionEnd(*other));
    }

    fn on_intersection_start(&mut self, other: &ColliderHandle) {
        let _send_result = self.inner.send(ColliderEvent::IntersectionStart(*other));
    }

    fn on_intersection_end(&mut self, other: &ColliderHandle) {
        let _send_result = self.inner.send(ColliderEvent::IntersectionEnd(*other));
    }

    fn on_contact_force_event(&mut self, other: &ColliderHandle, details: ContactForceEvent) {
        let _send_result = self
            .inner
            .send(ColliderEvent::ContactForceEvent(*other, details));
    }
}
