use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

use gltf::Gltf;

use crate::animation::asset::Animation;
use crate::ffi::{loop_animation, stop_animation};

#[derive(PartialEq, Eq)]
enum RepeatMode {
    Loop,
    None,
}

/// An animation asset with additional playback details.
pub struct DetailedAnimation {
    pub animation: Animation,
    pub time_scale: f32,
    pub is_active: bool,
    repeat_mode: RepeatMode,
}

impl DetailedAnimation {
    pub fn new(anim: Animation) -> Self {
        Self {
            animation: anim,
            time_scale: 1.0,
            is_active: false,
            repeat_mode: RepeatMode::None,
        }
    }
}

/// An animation mixer / manager capable of playing multiple simulataneous
/// animations with varying playback rates (including reverse playback).
#[derive(Default)]
pub struct AnimationManager {
    map: HashMap<String, DetailedAnimation>,
}

impl IntoIterator for AnimationManager {
    type Item = (String, DetailedAnimation);
    type IntoIter = IntoIter<String, DetailedAnimation>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl Extend<(String, DetailedAnimation)> for AnimationManager {
    fn extend<T: IntoIterator<Item = (String, DetailedAnimation)>>(&mut self, iter: T) {
        self.map.extend(iter);
    }
}

impl AnimationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn import_from_gltf(gltf: &Gltf) -> Self {
        let mut manager = Self::new();
        for anim in gltf.animations() {
            let animation_asset = Animation::from_gltf(
                gltf,
                anim.name()
                    .expect("Animation loaded from glTF doesn't have name"),
            )
            .expect("Animation that should exist, doesn't");
            manager.add(animation_asset);
        }
        manager
    }

    pub fn add(&mut self, anim: Animation) -> Option<DetailedAnimation> {
        self.map
            .insert(anim.name().clone(), DetailedAnimation::new(anim))
    }

    pub fn get(&self, name: &str) -> Option<&DetailedAnimation> {
        self.map.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut DetailedAnimation> {
        self.map.get_mut(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<DetailedAnimation> {
        self.map.remove(name)
    }

    pub fn update(&mut self, delta_seconds: f32) {
        for detailed_animation in self.map.values_mut().filter(|danim| danim.is_active) {
            let animation = &mut detailed_animation.animation;
            if detailed_animation.repeat_mode == RepeatMode::Loop {
                let new_timeline_pos =
                    animation.timeline_position() + delta_seconds * detailed_animation.time_scale;
                if new_timeline_pos > animation.duration() {
                    animation.set_timeline_position(new_timeline_pos - animation.duration());
                }
                if new_timeline_pos <= 0.0 {
                    animation.set_timeline_position(animation.duration() - new_timeline_pos);
                }
            }
            animation.update(delta_seconds * detailed_animation.time_scale);
        }
    }

    pub fn loop_animation(&mut self, anim_name: &str, scene_object_name: Option<&str>) {
        if let Some(detailed_anim) = self.get_mut(anim_name) {
            let scene_object_name = if let Some(name) = scene_object_name {
                name
            } else {
                ""
            };
            detailed_anim.is_active = true;
            detailed_anim.repeat_mode = RepeatMode::Loop;
            loop_animation(scene_object_name, anim_name, detailed_anim.time_scale);
        }
    }

    pub fn stop_animation(&mut self, anim_name: &str, scene_object_name: Option<&str>) {
        if let Some(detailed_anim) = self.get_mut(anim_name) {
            let scene_object_name = if let Some(name) = scene_object_name {
                name
            } else {
                ""
            };
            detailed_anim.is_active = false;
            stop_animation(scene_object_name, anim_name);
        }
    }
}
