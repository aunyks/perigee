// Much of this code is adapted from
// https://{github|gitlab}.com/aunyks/gltf-animation

use crate::perigee_gltf::util::access_gltf_bytes;
use crate::time::PassiveClock;
use gltf::{
    accessor::DataType as GltfDataType,
    animation::{Interpolation, Property as GltfProperty},
    Gltf,
};
use rapier3d::na::{Quaternion, UnitQuaternion, Vector3};
use std::collections::HashMap;
use thiserror::Error;

pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if (a - b).abs() > f32::EPSILON {
        return (value - a) / (b - a);
    }
    0.0
}

#[derive(Error, Debug)]
pub enum AnimationCreationError {
    #[error("could not find name in glTF document")]
    NameNotFound,
    #[error("theres not an even amount of keyframe timestamps for each keyframe property")]
    MismatchedKeyframes,
    #[error("could not find a binary blob in glTF document")]
    NoBinaryBlob,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ChannelType {
    Translation,
    Scale,
    Rotation,
}

#[derive(Debug, Clone, Copy)]
pub enum AnimatedProperty {
    Translation(Vector3<f32>),
    Scale(Vector3<f32>),
    Rotation(UnitQuaternion<f32>),
}

impl AnimatedProperty {
    pub fn inner_vector(&self) -> Option<Vector3<f32>> {
        match self {
            Self::Translation(vec) => Some(*vec),
            Self::Scale(vec) => Some(*vec),
            _ => None,
        }
    }

    pub fn inner_quaternion(&self) -> Option<UnitQuaternion<f32>> {
        match self {
            Self::Rotation(quat) => Some(*quat),
            _ => None,
        }
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Self::Translation(lhs_vec3), Self::Translation(rhs_vec3)) => {
                Self::Translation(lhs_vec3.lerp(rhs_vec3, t))
            }
            (Self::Scale(lhs_vec3), Self::Scale(rhs_vec3)) => {
                Self::Scale(lhs_vec3.lerp(rhs_vec3, t))
            }
            (Self::Rotation(lhs_quat), Self::Rotation(rhs_quat)) => {
                Self::Rotation(lhs_quat.slerp(rhs_quat, t))
            }
            _ => {
                panic!("Can't lerp between two different animation properties!")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Keyframe {
    timestamp: f32,
    property: AnimatedProperty,
}

impl Keyframe {
    pub fn new(timestamp: f32, property: AnimatedProperty) -> Self {
        Self {
            timestamp,
            property,
        }
    }

    pub fn timestamp(&self) -> f32 {
        self.timestamp
    }

    pub fn property(&self) -> AnimatedProperty {
        self.property
    }

    pub fn lerp_property(&self, other: &Self, timestamp: f32) -> AnimatedProperty {
        let t = inverse_lerp(self.timestamp, other.timestamp(), timestamp);
        self.property.lerp(&other.property(), t)
    }
}

#[derive(Debug, Clone)]
pub struct AnimationChannel {
    keyframes: Vec<Keyframe>,
    interpolation_method: Interpolation,
}

impl AnimationChannel {
    pub fn new(keyframes: Vec<Keyframe>, interpolation_method: Interpolation) -> Self {
        Self {
            keyframes,
            interpolation_method,
        }
    }

    pub fn keyframes(&self) -> &Vec<Keyframe> {
        &self.keyframes
    }

    pub fn duration(&self) -> f32 {
        // This should never panic since we guarantee tha ta channel
        // always has at least one keyframe
        self.keyframes
            .last()
            .expect("Could not get last keyframe of AnimationChannel. Channel has no keyframes")
            .timestamp()
    }

    pub fn interpolation_method(&self) -> Interpolation {
        self.interpolation_method
    }

    fn find_boundary_keyframes(&self, timestamp: f32) -> (Option<&Keyframe>, Option<&Keyframe>) {
        let first_keyframe = self
            .keyframes
            .first()
            .expect("Could not get first keyframe");
        let last_keyframe = self.keyframes.last().expect("Could not get last keyframe");
        if timestamp < first_keyframe.timestamp() {
            return (None, Some(&first_keyframe));
        }
        if timestamp > last_keyframe.timestamp() {
            return (Some(&last_keyframe), None);
        }
        let mut early_index: usize = 0;
        let mut late_index = self.keyframes.len() - 1;
        // Binary search
        while early_index <= late_index || (late_index - early_index) > 1 {
            if (late_index - early_index) <= 1 {
                return (
                    Some(&self.keyframes[early_index]),
                    Some(&self.keyframes[late_index]),
                );
            }
            let midpoint_index = (early_index + late_index) / 2;
            let midpoint_timestamp = self.keyframes[midpoint_index].timestamp();
            if timestamp > midpoint_timestamp {
                if midpoint_index == 0 {
                    return (
                        Some(&self.keyframes[midpoint_index]),
                        Some(&self.keyframes[1]),
                    );
                }
                early_index = midpoint_index + 1;
            } else if timestamp < midpoint_timestamp {
                late_index = midpoint_index - 1;
            } else {
                return (
                    Some(&self.keyframes[midpoint_index]),
                    Some(&self.keyframes[midpoint_index]),
                );
            }
        }
        unreachable!();
    }

    pub fn property_at(&self, timestamp: f32) -> AnimatedProperty {
        let (early_bound_frame, late_bound_frame) = self.find_boundary_keyframes(timestamp);
        if early_bound_frame.is_none() {
            return late_bound_frame
                .expect("Tried to get first keyframe but it doesn't exist")
                .property();
        }
        if late_bound_frame.is_none() {
            return early_bound_frame
                .expect("Tried to get last keyframe but it doesn't exist")
                .property();
        }
        let early_bound_frame =
            early_bound_frame.expect("Early bound keyframe was None despite asserting it wasn't.");
        let late_bound_frame =
            late_bound_frame.expect("Late bound keyframe was None despite asserting it wasn't.");
        // Interpolate between the two
        match self.interpolation_method {
            Interpolation::Step => {
                if (timestamp - late_bound_frame.timestamp()).abs() <= f32::EPSILON {
                    return late_bound_frame.property();
                }
                early_bound_frame.property()
            }
            Interpolation::Linear => early_bound_frame.lerp_property(&late_bound_frame, timestamp),
            Interpolation::CubicSpline => {
                early_bound_frame.lerp_property(&late_bound_frame, timestamp)
            }
        }
    }
}

/// A playable animation capable of having translation, rotation, and scale channels.
pub struct Animation {
    passive_timer: PassiveClock,
    fps: u32,
    duration: f32,
    target_channels: HashMap<String, HashMap<ChannelType, AnimationChannel>>,
    name: String,
    frame_listeners: HashMap<u32, Box<dyn FnMut()>>,
}

impl Animation {
    pub fn from_gltf(gltf: &Gltf, anim_name: &str) -> Result<Self, AnimationCreationError> {
        let gltf_bytes = match &gltf.blob {
            Some(bytes) => bytes,
            None => return Err(AnimationCreationError::NoBinaryBlob),
        };
        for anim in gltf.animations() {
            if let Some(anim_name_candidate) = anim.name() {
                if anim_name_candidate == anim_name {
                    let mut max_channel_duration = 0.0;
                    let mut max_channel_frames = 0;

                    let mut target_channels: HashMap<
                        String,
                        HashMap<ChannelType, AnimationChannel>,
                    > = HashMap::new();
                    for channel in anim.channels() {
                        if matches!(
                            channel.target().property(),
                            GltfProperty::MorphTargetWeights
                        ) {
                            continue;
                        }
                        let channel_sampler = channel.sampler();

                        let sampler_input = channel_sampler.input();
                        let sampler_input_bytes = access_gltf_bytes(&gltf_bytes, &sampler_input);

                        // If we have 0 timestamps then we can skip this iteration.
                        // One timestamp f32 would be 4 bytes, so if we have less than 4 bytes
                        // then we don't have a whole recoverable timestamp.
                        if sampler_input_bytes.len() < 4 {
                            continue;
                        }

                        if sampler_input.count() > max_channel_frames {
                            max_channel_frames = sampler_input.count();
                        }

                        let keyframe_timestamps: Vec<f32> = match channel_sampler.input().data_type() {
                            GltfDataType::F32 => sampler_input_bytes
                                .chunks_exact(4)
                                .map(|f32_bytes| {
                                    let f32_byte_array: [u8; 4] = f32_bytes[0..4]
                                        .try_into()
                                        .expect("Could not convert f32 byte slice into f32 byte array");
                                    f32::from_le_bytes(f32_byte_array)
                                })
                                .collect(),
                            _ => panic!(
                                "Unexpected data type received while converting sampler input bytes. Expected F32"
                            ),
                        };
                        if let Some(channel_duration) = keyframe_timestamps.last() {
                            if channel_duration > &max_channel_duration {
                                max_channel_duration = *channel_duration;
                            }
                        }

                        let sampler_output = channel_sampler.output();
                        let sampler_output_bytes = access_gltf_bytes(&gltf_bytes, &sampler_output);

                        let mut keyframe_properties: Vec<AnimatedProperty> =
                            Vec::with_capacity(keyframe_timestamps.len());

                        let channel_type: ChannelType;
                        match channel.target().property() {
                            GltfProperty::Translation => {
                                channel_type = ChannelType::Translation;
                                let flattened_vec3s: Vec<f32> = sampler_output_bytes
                                    .chunks_exact(4)
                                    .map(|f32_bytes| {
                                        let f32_byte_array: [u8; 4] = f32_bytes[0..4]
                                            .try_into()
                                            .expect(
                                            "Could not convert f32 byte slice into f32 byte array",
                                        );
                                        f32::from_le_bytes(f32_byte_array)
                                    })
                                    .collect();
                                flattened_vec3s.chunks_exact(3).for_each(|vec3| {
                                    keyframe_properties.push(AnimatedProperty::Translation(
                                        Vector3::new(vec3[0], vec3[1], vec3[2]),
                                    ))
                                });
                            }
                            GltfProperty::Scale => {
                                channel_type = ChannelType::Scale;
                                let flattened_vec3s: Vec<f32> = sampler_output_bytes
                                    .chunks_exact(4)
                                    .map(|f32_bytes| {
                                        let f32_byte_array: [u8; 4] = f32_bytes[0..4]
                                            .try_into()
                                            .expect(
                                            "Could not convert f32 byte slice into f32 byte array",
                                        );
                                        f32::from_le_bytes(f32_byte_array)
                                    })
                                    .collect();
                                flattened_vec3s.chunks_exact(3).for_each(|vec3| {
                                    keyframe_properties.push(AnimatedProperty::Scale(Vector3::new(
                                        vec3[0], vec3[1], vec3[2],
                                    )))
                                });
                            }
                            GltfProperty::Rotation => {
                                channel_type = ChannelType::Rotation;
                                let flattened_vec4s: Vec<f32> = sampler_output_bytes
                                    .chunks_exact(4)
                                    .map(|f32_bytes| {
                                        let f32_byte_array: [u8; 4] = f32_bytes[0..4]
                                            .try_into()
                                            .expect(
                                            "Could not convert f32 byte slice into f32 byte array",
                                        );
                                        f32::from_le_bytes(f32_byte_array)
                                    })
                                    .collect();
                                flattened_vec4s.chunks_exact(4).for_each(|vec4| {
                                    keyframe_properties.push(AnimatedProperty::Rotation(
                                        UnitQuaternion::from_quaternion(Quaternion::new(
                                            vec4[3], vec4[0], vec4[1], vec4[2],
                                        )),
                                    ))
                                });
                            }
                            _ => {
                                unreachable!();
                            }
                        };

                        if keyframe_timestamps.len() % keyframe_properties.len() != 0 {
                            return Err(AnimationCreationError::MismatchedKeyframes);
                        }

                        let frames_per_property =
                            keyframe_timestamps.len() / keyframe_properties.len();
                        let mut keyframes: Vec<Keyframe> =
                            Vec::with_capacity(keyframe_properties.len());
                        for i in (0..keyframe_timestamps.len()).step_by(frames_per_property) {
                            keyframes.push(Keyframe::new(
                                keyframe_timestamps[i],
                                keyframe_properties[i / frames_per_property],
                            ));
                        }

                        let new_channel =
                            AnimationChannel::new(keyframes, channel_sampler.interpolation());

                        let target_name =
                            String::from(channel.target().node().name().unwrap_or("Unknown"));
                        if let Some(channels_for_target) = target_channels.get_mut(&target_name) {
                            channels_for_target.insert(channel_type, new_channel);
                        } else {
                            target_channels
                                .insert(target_name, HashMap::from([(channel_type, new_channel)]));
                        }
                    }

                    return Ok(Self {
                        passive_timer: PassiveClock::default(),
                        target_channels,
                        fps: (max_channel_frames as f32 / max_channel_duration).round() as u32,
                        duration: max_channel_duration,
                        name: String::from(anim_name),
                        frame_listeners: HashMap::new(),
                    });
                }
            }
        }
        return Err(AnimationCreationError::NameNotFound);
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn frame_at_time(&self, timestamp: f32) -> u32 {
        (timestamp * self.fps() as f32).round() as u32
    }

    pub fn on_frame(&mut self, frame: u32, listener: impl FnMut() + 'static) {
        self.frame_listeners.insert(frame, Box::new(listener));
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn set_timeline_position(&mut self, secs: f32) {
        self.passive_timer.set_seconds(secs);
    }

    pub fn timeline_position(&self) -> f32 {
        self.passive_timer.elapsed().as_secs_f32()
    }

    pub fn update(&mut self, delta_seconds: f32) {
        if delta_seconds >= 0.0 {
            self.update_forward(delta_seconds);
        } else {
            self.update_reverse(delta_seconds);
        }
    }

    fn update_forward(&mut self, delta_seconds: f32) {
        let old_frame = self.frame_at_time(self.timeline_position());
        self.passive_timer.tick(delta_seconds);
        let new_frame = self.frame_at_time(self.timeline_position());

        for (listener_frame, listener) in self.frame_listeners.iter_mut() {
            if listener_frame <= &old_frame {
                continue;
            }
            if listener_frame > &new_frame {
                continue;
            }

            listener();
        }
    }

    fn update_reverse(&mut self, delta_seconds: f32) {
        let old_frame = self.frame_at_time(self.timeline_position());
        self.passive_timer.tick_reverse(delta_seconds);
        let new_frame = self.frame_at_time(self.timeline_position());

        for (listener_frame, listener) in self.frame_listeners.iter_mut() {
            if listener_frame >= &old_frame {
                continue;
            }
            if listener_frame < &new_frame {
                continue;
            }

            listener();
        }
    }

    pub fn current_translation(&self, target_name: &str) -> Option<Vector3<f32>> {
        match self.target_channels.get(target_name) {
            Some(channels) => match channels.get(&ChannelType::Translation) {
                Some(channel) => Some(
                    channel
                        .property_at(self.passive_timer.elapsed().as_secs_f32())
                        .inner_vector()
                        .expect("Tried to get inner vector of Translation property but it doesn't exist"),
                ),
                None => None,
            },
            None => None,
        }
    }

    pub fn current_scale(&self, target_name: &str) -> Option<Vector3<f32>> {
        match self.target_channels.get(target_name) {
            Some(channels) => match channels.get(&ChannelType::Scale) {
                Some(channel) => Some(
                    channel
                        .property_at(self.passive_timer.elapsed().as_secs_f32())
                        .inner_vector()
                        .expect("Tried to get inner vector of Scale property but it doesn't exist"),
                ),
                None => None,
            },
            None => None,
        }
    }

    pub fn current_rotation(&self, target_name: &str) -> Option<UnitQuaternion<f32>> {
        match self.target_channels.get(target_name) {
            Some(channels) => match channels.get(&ChannelType::Rotation) {
                Some(channel) => Some(
                    channel
                        .property_at(self.passive_timer.elapsed().as_secs_f32())
                        .inner_quaternion()
                        .expect("Tried to get inner quaternion of Rotation property but it doesn't exist"),
                ),
                None => None,
            },
            None => None,
        }
    }
}
