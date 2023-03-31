use rapier3d::prelude::*;

pub trait ColliderEventListener {
    fn on_collision_start(&mut self, _other: &ColliderHandle) {}
    fn on_collision_end(&mut self, _other: &ColliderHandle) {}
    fn on_intersection_start(&mut self, _other: &ColliderHandle) {}
    fn on_intersection_end(&mut self, _other: &ColliderHandle) {}
    fn on_contact_force_event(&mut self, _other: &ColliderHandle, _details: ContactForceEvent) {}
}
