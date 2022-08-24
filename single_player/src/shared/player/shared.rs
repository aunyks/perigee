use crate::shared::interactions::InteractionGroup;
use perigee_core::prelude::*;
use serde::{Deserialize, Serialize};

pub const COLLIDER_RAYCAST_OFFSET: f32 = 0.001;
pub static INVALID_VECTOR: Vector3<f32> = Vector3::new(f32::NAN, f32::NAN, f32::NAN);
pub static UP_VECTOR: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
pub static DOWN_VECTOR: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
pub static RIGHT_VECTOR: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
pub static LEFT_VECTOR: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
pub static FORWARD_VECTOR: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
pub static BACK_VECTOR: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrouchState {
    Upright,
    Crouched,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlidingState {
    None,
    Normal,
    Downhill,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum WallRunning {
    OnRight(Vector3<f32>),
    OnLeft(Vector3<f32>),
    None,
}

impl PartialEq for WallRunning {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (WallRunning::OnLeft(_), WallRunning::OnLeft(_))
                | (WallRunning::OnRight(_), WallRunning::OnRight(_))
                | (WallRunning::None, WallRunning::None)
        )
    }
}

impl Eq for WallRunning {}

pub fn query_filter_excluding_player() -> QueryFilter<'static> {
    QueryFilter {
        groups: Some(InteractionGroups::all().with_filter(
            Group::from_bits_truncate(u32::from(InteractionGroup::Player)).complement(),
        )),
        ..Default::default()
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Boom {
    pub translation: Translation<f32>,
    pub z_rotation: UnitQuaternion<f32>,
    pub x_rotation: UnitQuaternion<f32>,
    arm_pivot: Isometry<f32, UnitQuaternion<f32>, 3>,
    length: f32,
}

impl std::fmt::Debug for Boom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.arm_pivot.fmt(f)
    }
}

impl Boom {
    pub fn new(length: f32, arm_pitch_angle: f32, arm_yaw_angle: f32) -> Self {
        let mut new_boom = Self::default();
        new_boom.set_length(length);

        new_boom.arm_pivot.rotation = UnitQuaternion::from_euler_angles(
            arm_pitch_angle.to_radians(),
            arm_yaw_angle.to_radians(),
            0.0,
        );

        new_boom
    }

    pub fn set_length(&mut self, new_length: f32) {
        self.length = new_length
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn isometry(&self) -> Isometry<f32, UnitQuaternion<f32>, 3> {
        Isometry::from_parts(self.translation, self.z_rotation * self.x_rotation)
    }

    pub fn end_isometry(&self) -> Isometry<f32, UnitQuaternion<f32>, 3> {
        self.isometry()
            * self.arm_pivot
            * Isometry::from_parts(
                Translation::from(Vector3::new(0.0, 0.0, self.length)),
                self.arm_pivot.rotation.inverse(),
            )
    }

    pub fn lerp_mut(&mut self, other: &Self, t: f32) {
        self.length = lerp(self.length(), other.length(), t);
        self.arm_pivot = self.arm_pivot.lerp_slerp(&other.arm_pivot, t);
        // debug!(
        //     "t: {:?} / T: {:?} / L: {:?}",
        //     t, &other.arm_pivot, self.arm_pivot
        // );
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PerspectiveMode {
    FirstPerson,
    ThirdPersonBasic,
    ThirdPersonCombat,
}

impl Default for PerspectiveMode {
    fn default() -> Self {
        Self::ThirdPersonBasic
    }
}

impl PerspectiveMode {
    pub fn is_third_person(&self) -> bool {
        self == &PerspectiveMode::ThirdPersonBasic || self == &PerspectiveMode::ThirdPersonCombat
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MovementMode {
    Discrete,
    Continuous,
}

impl Default for MovementMode {
    fn default() -> Self {
        Self::Discrete
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalkDirection {
    Forward,
    RightForward,
    Right,
    RightBack,
    Back,
    LeftBack,
    Left,
    LeftForward,
}

impl Default for WalkDirection {
    fn default() -> Self {
        Self::Forward
    }
}

impl ToString for WalkDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Forward => "FORWARD",
            Self::RightForward => "RIGHT_FORWARD",
            Self::Right => "RIGHT",
            Self::RightBack => "RIGHT_BACK",
            Self::Back => "BACKWARD",
            Self::LeftBack => "LEFT_BACK",
            Self::Left => "LEFT",
            Self::LeftForward => "LEFT_FORWARD",
        }
        .to_string()
    }
}

impl WalkDirection {
    pub fn from_movement_vector(movement_vector: &Vector3<f32>) -> Option<Self> {
        if movement_vector.magnitude() <= 0.0 {
            return None;
        }
        return Some(
            if movement_vector.angle(&FORWARD_VECTOR).to_degrees() < 22.5 {
                WalkDirection::Forward
            } else if movement_vector
                .angle(&(RIGHT_VECTOR + FORWARD_VECTOR))
                .to_degrees()
                <= 22.5
            {
                WalkDirection::RightForward
            } else if movement_vector.angle(&RIGHT_VECTOR).to_degrees() < 22.5 {
                WalkDirection::Right
            } else if movement_vector
                .angle(&(RIGHT_VECTOR + BACK_VECTOR))
                .to_degrees()
                <= 22.5
            {
                WalkDirection::RightBack
            } else if movement_vector.angle(&BACK_VECTOR).to_degrees() < 22.5 {
                WalkDirection::Back
            } else if movement_vector
                .angle(&(LEFT_VECTOR + BACK_VECTOR))
                .to_degrees()
                <= 22.5
            {
                WalkDirection::LeftBack
            } else if movement_vector.angle(&LEFT_VECTOR).to_degrees() < 22.5 {
                WalkDirection::Left
            } else {
                WalkDirection::LeftForward
            },
        );
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Eq, Debug)]
pub enum MovementState {
    Stationary(CrouchState),
    Creeping,
    Walking(WalkDirection),
    Running,
    Sprinting,
    InAir,
}

impl PartialEq for MovementState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sprinting, Self::Sprinting) => true,
            (Self::Running, Self::Running) => true,
            (Self::Creeping, Self::Creeping) => true,
            (Self::Stationary(self_crouch_state), Self::Stationary(other_crouch_state)) => {
                self_crouch_state == other_crouch_state
            }
            (Self::Walking(self_walk_dir), Self::Walking(other_walk_dir)) => {
                self_walk_dir == other_walk_dir
            }
            (Self::InAir, Self::InAir) => true,
            _ => false,
        }
    }
}

impl Default for MovementState {
    fn default() -> Self {
        Self::Stationary(CrouchState::Upright)
    }
}

impl ToString for MovementState {
    fn to_string(&self) -> String {
        match self {
            Self::Stationary(crouch_state) => match crouch_state {
                CrouchState::Upright => "IDLE".to_string(),
                CrouchState::Crouched => "CROUCHED".to_string(),
            },
            Self::Creeping => "CREEPING".to_string(),
            Self::Walking(walk_dir) => format!("WALK_{}", walk_dir.to_string()),
            Self::Running => "RUN_FORWARD".to_string(),
            Self::Sprinting => "SPRINT_FORWARD".to_string(),
            Self::InAir => "IN_AIR".to_string(),
        }
    }
}
