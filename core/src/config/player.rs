use serde::{Deserialize, Serialize};

// Note: #[serde(default)] means that if the
//       field isn't defined during deserialization
//       then its `Default` value is used.
//
//       #[serde(skip)] means that we skip the field
//       during serialization and don't include it in the output.

/// Configuration parameters for the [Player](crate::player::Player).
/// These should not be editable at runtime.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PlayerConfig {
    /// The mass of the player's body (via its collider).
    #[serde(default)]
    pub mass: f32,
    /// How high the player can look (max X axis rotation of the head or viewpoint).
    #[serde(default)]
    pub max_look_up_angle: f32,
    /// How low the player can look (min X axis rotation of the head or viewpoint).
    #[serde(default)]
    pub min_look_up_angle: f32,
    /// The per-frame lerp factor (alpha) used when entering a wallrunning head tilt.
    #[serde(default)]
    pub enter_head_tilt_factor: f32,
    /// The per-frame lerp factor (alpha) used when exiting a wallrunning head tilt.
    #[serde(default)]
    pub exit_head_tilt_factor: f32,
    /// How fast the player must be moving to be considered
    /// moving.
    #[serde(default)]
    pub nonstationary_speed_threshold: f32,
    /// The max speed to which player's own forces can move its rigid body when standing.
    #[serde(default)]
    pub max_standing_move_speed: f32,
    /// The max speed to which player's own forces can move its rigid body when crouched.
    #[serde(default)]
    pub max_crouched_move_speed: f32,
    /// The max acceleration force the player can apply to its rigid body when standing.
    #[serde(default)]
    pub max_standing_move_acceleration: f32,
    /// The max acceleration force the player can apply to its rigid body when crouched.
    #[serde(default)]
    pub max_crouched_move_acceleration: f32,
    /// The total height of the player's capsule when standing.
    #[serde(default)]
    pub capsule_standing_height: f32,
    /// The radius of the cylinder part of the player and each half-sphere of the capsule when standing.
    #[serde(default)]
    pub capsule_standing_radius: f32,
    /// The total height of the player's capsule when crouched.
    #[serde(default)]
    pub capsule_crouched_height: f32,
    /// The radius of the cylinder part of the player and each half-sphere of the capsule when standing.
    #[serde(default)]
    pub capsule_crouched_radius: f32,
    /// The translational offset of the head (which is often tracked by the camera) from the center of the
    /// capsule when standing.
    #[serde(default)]
    pub standing_head_translation_offset: [f32; 3],
    /// The translational offset of the head (which is often tracked by the camera) from the center of the
    /// capsule when crouched.
    #[serde(default)]
    pub crouched_head_translation_offset: [f32; 3],
    /// How quickly the player's head lerps between its standing
    /// translational offset and crouched translational offset.
    #[serde(default)]
    pub head_crouch_lerp_factor: f32,
    /// How many seconds after no longer being grounded or wallrunning
    /// the player can still jump.
    #[serde(default)]
    pub max_jump_coyote_duration: f32,
    /// How much force is used to make the player jump when standing.
    #[serde(default)]
    pub jump_standing_acceleration: f32,
    /// How much force is used to make the player jump when crouched.
    #[serde(default)]
    pub jump_crouched_acceleration: f32,
    /// How many seconds must pass before another jump is possible
    /// while the player is standing up.
    #[serde(default)]
    pub min_jump_standing_cooldown_duration: f32,
    /// How many seconds must pass before another jump is possible.
    /// while the player is crouching.
    #[serde(default)]
    pub min_jump_crouched_cooldown_duration: f32,
    /// The scale factor of jump force when wallrunning.
    #[serde(default)]
    pub jump_wallrunning_scale: f32,
    /// How far from the player the rays used to determine whether it's
    /// wallrunning go.
    #[serde(default)]
    pub wallrunning_ray_length: f32,
    /// How far below the player the ray used to determine whether it's
    /// grounded goes.
    #[serde(default)]
    pub ground_ray_length: f32,
    /// How far straight ahead the player must be moving next to a wall
    /// to be considered wallrunning. Values closer to 1 mean more straightforwardness.
    #[serde(default)]
    pub wallrunning_dot_value: f32,
    /// The vertical acceleration applied to the player's
    /// body once wallrunning has started.
    #[serde(default)]
    pub start_wallrunning_up_acceleration: f32,
    /// The gravity scale of the player's body once wallrunning
    /// has started.
    #[serde(default)]
    pub start_wallrunning_gravity_scale: f32,
    /// How many seconds should pass before another footstep is taken
    /// when moving at the max speed while grounded.
    #[serde(default)]
    pub grounded_seconds_per_footstep: f32,
    /// How many seconds should pass before another footstep is taken
    /// when moving at the max speed while wallrunning.
    #[serde(default)]
    pub wallrunning_seconds_per_footstep: f32,
    /// How much of the max standing speed must the player
    /// be moving in order to slide when the crouch input is hit.
    #[serde(default)]
    pub sliding_speed_factor: f32,
    /// How straightforward the player must be moving
    /// before entering a slide.
    #[serde(default)]
    pub sliding_forward_factor: f32,
    /// The acceleration vector applied to the rigid body
    /// when the player starts sliding.
    #[serde(default)]
    pub sliding_deceleration: [f32; 3],
    /// The increase in velocity applied to the rigid
    /// body when the player starts sliding.
    #[serde(default)]
    pub sliding_velocity_increase: [f32; 3],
}

impl Default for PlayerConfig {
    fn default() -> Self {
        let capsule_total_height = 1.83;
        let capsule_radius = 0.4;
        Self {
            mass: 1.0,
            max_look_up_angle: std::f32::consts::FRAC_PI_2,
            min_look_up_angle: -std::f32::consts::FRAC_PI_2
                + (30.0 * (std::f32::consts::FRAC_PI_2 / 180.0)),
            enter_head_tilt_factor: 0.12,
            exit_head_tilt_factor: 0.08,
            nonstationary_speed_threshold: 0.01,
            max_standing_move_speed: 7.5,
            max_crouched_move_speed: 2.5,
            max_standing_move_acceleration: 25.0,
            max_crouched_move_acceleration: 12.5,
            capsule_standing_height: capsule_total_height,
            capsule_standing_radius: capsule_radius,
            capsule_crouched_height: capsule_total_height / 2.0,
            capsule_crouched_radius: capsule_radius,
            standing_head_translation_offset: [
                0.0,
                capsule_total_height / 2.0 * 0.84,
                -capsule_radius * 0.8,
            ],
            crouched_head_translation_offset: [
                0.0,
                capsule_total_height / 4.0 * 0.84,
                -capsule_radius * 0.8,
            ],
            head_crouch_lerp_factor: 0.4,
            max_jump_coyote_duration: 0.275,
            jump_standing_acceleration: 6.0,
            jump_crouched_acceleration: 3.5,
            min_jump_standing_cooldown_duration: 0.3,
            min_jump_crouched_cooldown_duration: 0.5,
            jump_wallrunning_scale: 0.35,
            ground_ray_length: 0.1,
            wallrunning_ray_length: 0.4,
            wallrunning_dot_value: 0.5,
            start_wallrunning_up_acceleration: 4.0,
            start_wallrunning_gravity_scale: 0.5,
            grounded_seconds_per_footstep: 1.0 / 4.0,
            wallrunning_seconds_per_footstep: 1.0 / 6.0,
            sliding_speed_factor: 0.8,
            sliding_forward_factor: 0.8,
            sliding_velocity_increase: [0.0, 0.0, -6.0],
            sliding_deceleration: [0.0, 0.0, 4.5],
        }
    }
}

impl PlayerConfig {
    /// The mass of the player's body (via its collider).
    pub fn mass(&self) -> f32 {
        self.mass
    }

    /// How high the player can look (max X axis rotation of the head or viewpoint).
    pub fn max_look_up_angle(&self) -> f32 {
        self.max_look_up_angle
    }

    /// How low the player can look (min X axis rotation of the head or viewpoint).
    pub fn min_look_up_angle(&self) -> f32 {
        self.min_look_up_angle
    }

    /// The per-frame lerp factor (alpha) used when entering a wallrunning head tilt.
    pub fn enter_head_tilt_factor(&self) -> f32 {
        self.enter_head_tilt_factor
    }

    /// The per-frame lerp factor (alpha) used when exiting a wallrunning head tilt.
    pub fn exit_head_tilt_factor(&self) -> f32 {
        self.exit_head_tilt_factor
    }

    /// How fast the player must be moving to be considered
    /// moving.
    pub fn nonstationary_speed_threshold(&self) -> f32 {
        self.nonstationary_speed_threshold
    }

    /// The max speed to which player's own forces can move its rigid body when standing.
    pub fn max_standing_move_speed(&self) -> f32 {
        self.max_standing_move_speed
    }

    /// The max speed to which player's own forces can move its rigid body when crouched.
    pub fn max_crouched_move_speed(&self) -> f32 {
        self.max_crouched_move_speed
    }

    /// The max acceleration force the player can apply to its rigid body when standing.
    pub fn max_standing_move_acceleration(&self) -> f32 {
        self.max_standing_move_acceleration
    }

    /// The max acceleration force the player can apply to its rigid body when crouched.
    pub fn max_crouched_move_acceleration(&self) -> f32 {
        self.max_crouched_move_acceleration
    }

    /// The total height of the player's capsule when standing.
    pub fn capsule_standing_total_height(&self) -> f32 {
        self.capsule_standing_height
    }

    /// The height of the cylinder part of the player's capsule when standing.
    pub fn capsule_standing_half_height(&self) -> f32 {
        self.capsule_standing_height / 2.0 - self.capsule_standing_radius
    }

    /// The radius of the cylinder part of the player and each half-sphere of the capsule when standing.
    pub fn capsule_standing_radius(&self) -> f32 {
        self.capsule_standing_radius
    }

    /// The total height of the player's capsule when crouched.
    pub fn capsule_crouched_total_height(&self) -> f32 {
        self.capsule_crouched_height
    }

    /// The height of the cylinder part of the player's capsule when crouched.
    pub fn capsule_crouched_half_height(&self) -> f32 {
        self.capsule_crouched_height / 2.0 - self.capsule_crouched_radius
    }

    /// The radius of the cylinder part of the player and each half-sphere of the capsule when crouched.
    pub fn capsule_crouched_radius(&self) -> f32 {
        self.capsule_crouched_radius
    }

    /// The translational offset of the head (which is often tracked by the camera) from the center of the
    /// capsule when standing.
    pub fn standing_head_translation_offset(&self) -> [f32; 3] {
        self.standing_head_translation_offset
    }

    /// The translational offset of the head (which is often tracked by the camera) from the center of the
    /// capsule when crouched.
    pub fn crouched_head_translation_offset(&self) -> [f32; 3] {
        self.crouched_head_translation_offset
    }

    /// How quickly the player's head lerps between its standing
    /// translational offset and crouched translational offset.
    pub fn head_crouch_lerp_factor(&self) -> f32 {
        self.head_crouch_lerp_factor
    }

    /// How many seconds after no longer being grounded or wallrunning
    /// the player can still jump.
    pub fn max_jump_coyote_duration(&self) -> f32 {
        self.max_jump_coyote_duration
    }

    /// How much force is used to make the player jump when standing.
    pub fn jump_standing_acceleration(&self) -> f32 {
        self.jump_standing_acceleration
    }

    /// How much force is used to make the player jump when crouched.
    pub fn jump_crouched_acceleration(&self) -> f32 {
        self.jump_crouched_acceleration
    }

    /// How many seconds must pass before another jump is possible
    /// while the player is standing up.
    pub fn min_jump_standing_cooldown_duration(&self) -> f32 {
        self.min_jump_standing_cooldown_duration
    }

    /// How many seconds must pass before another jump is possible
    /// while the player is crouching.
    pub fn min_jump_crouched_cooldown_duration(&self) -> f32 {
        self.min_jump_crouched_cooldown_duration
    }

    /// The scale factor of jump force when wallrunning.
    pub fn jump_wallrunning_scale(&self) -> f32 {
        self.jump_wallrunning_scale
    }

    /// How far below the player the ray used to determine whether it's
    /// grounded goes.
    pub fn ground_ray_length(&self) -> f32 {
        self.ground_ray_length
    }

    /// How far from the player the rays used to determine whether it's
    /// wallrunning go.
    pub fn wallrunning_ray_length(&self) -> f32 {
        self.wallrunning_ray_length
    }

    /// How far straight ahead the player must be moving next to a wall
    /// to be considered wallrunning. Values closer to 1 mean more straightforwardness.
    pub fn wallrunning_dot_value(&self) -> f32 {
        self.wallrunning_dot_value
    }

    /// The vertical acceleration applied to the player's
    /// body once wallrunning has started.
    pub fn start_wallrunning_up_acceleration(&self) -> f32 {
        self.start_wallrunning_up_acceleration
    }

    /// The gravity scale of the player's body once wallrunning
    /// has started.
    pub fn start_wallrunning_gravity_scale(&self) -> f32 {
        self.start_wallrunning_gravity_scale
    }

    /// How many seconds should pass before another footstep is taken
    /// when moving at the max speed while grounded.
    pub fn grounded_seconds_per_footstep(&self) -> f32 {
        self.grounded_seconds_per_footstep
    }

    /// How many seconds should pass before another footstep is taken
    /// when moving at the max speed while wallrunning.
    pub fn wallrunning_seconds_per_footstep(&self) -> f32 {
        self.wallrunning_seconds_per_footstep
    }

    /// How much of the max standing speed must the player
    /// be moving in order to slide when the crouch input is hit.
    pub fn sliding_speed_factor(&self) -> f32 {
        self.sliding_speed_factor
    }

    /// How straightforward the player must be moving
    /// before entering a slide.
    pub fn sliding_forward_factor(&self) -> f32 {
        self.sliding_forward_factor
    }

    /// The deceleration vector applied to the rigid body
    /// when the player starts sliding.
    pub fn sliding_deceleration(&self) -> [f32; 3] {
        self.sliding_deceleration
    }

    /// The increase in velocity applied to the rigid
    /// body when the player starts sliding.
    pub fn sliding_velocity_increase(&self) -> [f32; 3] {
        self.sliding_velocity_increase
    }
}
