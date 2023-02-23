use serde::{Deserialize, Serialize};

/// Configuration parameters for the [FirstPersonPlayer](crate::shared::player::FirstPersonPlayer).
/// These should not be editable at runtime.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PlayerConfig {
    /// The mass of the player's body (via its collider).
    mass: f32,
    /// How high the player can look (max X axis rotation of the head or viewpoint).
    max_look_up_angle: f32,
    /// How low the player can look (min X axis rotation of the head or viewpoint).
    min_look_up_angle: f32,
    /// The per-frame lerp factor (alpha) used when entering a wallrunning head tilt.
    enter_head_tilt_factor: f32,
    /// The per-frame lerp factor (alpha) used when exiting a wallrunning head tilt.
    exit_head_tilt_factor: f32,
    /// How fast the player must be moving to be considered
    /// moving.
    nonstationary_speed_threshold: f32,
    /// The max speed to which player's own forces can move its rigid body when standing in the continuous movement mode.
    max_standing_move_speed_continuous: f32,
    /// The max speed to which player's own forces can move its rigid body when crouched in the continuous movement mode.
    max_crouched_move_speed_continuous: f32,
    /// The max acceleration force the player can apply to its rigid body when standing in the continuous movement mode.
    max_standing_move_acceleration_continuous: f32,
    /// The max acceleration force the player can apply to its rigid body when crouched in the continuous movement mode.
    max_crouched_move_acceleration_continuous: f32,
    /// The walk speed of the player when standing in the discrete movement mode.
    standing_walk_speed_discrete: f32,
    /// The run speed of the player when standing in the discrete movement mode.
    standing_run_speed_discrete: f32,
    /// The sprint speed of the player when standing in the discrete movement mode.
    standing_sprint_speed_discrete: f32,
    /// The creep speed of the player when crouched in the discrete movement mode.
    crouched_creep_speed_discrete: f32,
    /// The walk acceleration force the player applies to its rigid body when standing in the discrete movement mode.
    standing_walk_acceleration_discrete: f32,
    /// The run acceleration force the player applies to its rigid body when standing in the discrete movement mode.
    standing_run_acceleration_discrete: f32,
    /// The sprint acceleration force the player applies to its rigid body when standing in the discrete movement mode.
    standing_sprint_acceleration_discrete: f32,
    /// The creep acceleration force the player applies to its rigid body when crouched in the discrete movement mode.
    crouched_creep_acceleration_discrete: f32,
    /// The threshold (between 0 and 1) above which a player's movement vector's magnitude must be greater than to trigger a sprint.
    standing_sprint_input_threshold: f32,
    /// The max angle between the forward vector and the movement vector under which the player can sprint in discrete movement mode.
    max_sprint_forward_angle_threshold_discrete: f32,

    discrete_movement_factor: f32,
    /// The threshold (between 0 and 1) above which a player's movement vector's magnitude must be greater than to trigger a run.
    standing_run_input_threshold: f32,
    /// The total height of the player's capsule when standing.
    capsule_standing_height: f32,
    /// The radius of the cylinder part of the player and each half-sphere of the capsule when standing.
    capsule_standing_radius: f32,
    /// The total height of the player's capsule when crouched.
    capsule_crouched_height: f32,
    /// The radius of the cylinder part of the player and each half-sphere of the capsule when standing.
    capsule_crouched_radius: f32,
    /// The translational offset of the head (which is often tracked by the camera) from the center of the
    /// capsule when standing.
    standing_head_translation_offset: [f32; 3],
    /// The translational offset of the head (which is often tracked by the camera) from the center of the
    /// capsule when crouched.
    crouched_head_translation_offset: [f32; 3],
    /// How quickly the player's head lerps between its standing
    /// translational offset and crouched translational offset.
    head_crouch_lerp_factor: f32,
    /// How many seconds after no longer being grounded or wallrunning
    /// the player can still jump.
    max_jump_coyote_duration: f32,
    /// How much force is used to make the player jump when standing.
    jump_standing_acceleration: f32,
    /// How much force is used to make the player jump when crouched.
    jump_crouched_acceleration: f32,
    /// How many seconds must pass before another jump is possible
    /// while the player is standing up.
    min_jump_standing_cooldown_duration: f32,
    /// How many seconds must pass before another jump is possible.
    /// while the player is crouching.
    min_jump_crouched_cooldown_duration: f32,
    /// The scale factor of jump force (up + forward) when wallrunning.
    jump_wallrunning_scale: f32,
    /// How close to straight down the body must be moving when wallrunning for the
    /// vertical velocity to be canceled before jumping off the wall.
    ///
    /// This is the minimum angle between the velocity and the down vector
    /// to be considered wallrunning downward.
    jump_wallrunning_down_velocity_angle_threshold: f32,
    /// The scale factor of jump force in the direction of the wall normal when wallrunning.
    jump_wallrunning_normal_scale: f32,
    /// How far from the player the rays used to determine whether it's
    /// wallrunning go.
    wallrunning_ray_length: f32,
    /// How far below the player the ray used to determine whether it's
    /// grounded goes.
    ground_ray_length: f32,
    /// How far straight ahead the player must be moving next to a wall
    /// to be considered wallrunning. Values closer to 1 mean more straightforwardness.
    max_wallrunning_forward_angle: f32,
    /// The vertical acceleration applied to the player's
    /// body once wallrunning has started.
    start_wallrunning_up_impulse: f32,
    /// The gravity scale of the player's body once wallrunning
    /// has started.
    start_wallrunning_gravity_scale: f32,
    /// How many seconds should pass before another footstep is taken
    /// when moving at the max speed while grounded.
    grounded_seconds_per_footstep: f32,
    /// How many seconds should pass before another footstep is taken
    /// when moving at the max speed while wallrunning.
    wallrunning_seconds_per_footstep: f32,
    /// How much of the max standing speed must the player
    /// be moving in order to slide when the crouch input is hit.
    sliding_speed_factor: f32,
    /// How straightforward the player must be moving
    /// before entering a slide.
    sliding_max_forward_angle: f32,
    /// The acceleration vector applied to the rigid body
    /// when the player starts sliding.
    sliding_deceleration: [f32; 3],
    /// The increase in velocity applied to the rigid
    /// body when the player starts sliding.
    sliding_velocity_increase: [f32; 3],
    /// The minimum dot factor of the player's velocity with
    /// a vector facing (0, -1, -1) needed for the player to be
    /// considered traveling downhill.
    endless_slide_downhill_max_down_angle: f32,
    /// The maximum dot factor of the player's ground normal
    /// with the up vector (0, 1, 0) to be considered traveling downhill.
    endless_slide_ground_normal_max_up_angle: f32,
    /// The acceleration applied to endless / downhill slides.
    endless_sliding_acceleration: [f32; 3],
    /// The max capacity of the event channel used by the player structure.
    event_queue_capacity: usize,
    /// The length of the default boom arm.
    default_boom_arm_length: f32,
    /// The pitch angle (about X axis) of the default boom arm in degrees.
    default_boom_arm_pitch_angle: f32,
    /// The yaw angle (about Y axis) of the default boom arm in degrees.
    default_boom_arm_yaw_angle: f32,
    /// How quickly the third person boom moves between the default and aim booms.
    boom_lerp_factor: f32,
    /// The length of the aiming boom arm.
    aim_boom_arm_length: f32,
    /// The pitch angle (about X axis) of the aiming boom arm in degrees.
    aim_boom_arm_pitch_angle: f32,
    /// The yaw angle (about Y axis) of the aiming boom arm in degrees.
    aim_boom_arm_yaw_angle: f32,
    /// The lerp factor of player body isometry to the boom isometry
    /// while in third person combat mode.
    tpcombat_boom_rotation_lerp_factor: f32,
    /// The lerp factor for the player body to rotate in the player's movement direction.
    rotate_body_to_movement_dir_lerp_factor: f32,
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
            nonstationary_speed_threshold: 0.02,
            max_standing_move_speed_continuous: 5.0,
            max_crouched_move_speed_continuous: 2.5,
            max_standing_move_acceleration_continuous: 25.0,
            max_crouched_move_acceleration_continuous: 12.5,
            standing_walk_speed_discrete: 3.0,
            standing_run_speed_discrete: 5.0,
            standing_sprint_speed_discrete: 7.5,
            crouched_creep_speed_discrete: 1.0,
            standing_walk_acceleration_discrete: 30.0,
            standing_run_acceleration_discrete: 35.0,
            standing_sprint_acceleration_discrete: 40.0,
            crouched_creep_acceleration_discrete: 28.0,
            standing_sprint_input_threshold: 0.9,
            max_sprint_forward_angle_threshold_discrete: 22.4,
            standing_run_input_threshold: 0.5,
            discrete_movement_factor: 0.75,
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
            head_crouch_lerp_factor: 0.2,
            max_jump_coyote_duration: 0.275,
            jump_standing_acceleration: 6.0,
            jump_crouched_acceleration: 3.5,
            min_jump_standing_cooldown_duration: 0.3,
            min_jump_crouched_cooldown_duration: 0.5,
            jump_wallrunning_scale: 1.0,
            jump_wallrunning_normal_scale: 0.35,
            jump_wallrunning_down_velocity_angle_threshold: 30.0,
            ground_ray_length: 0.1,
            wallrunning_ray_length: 0.4,
            max_wallrunning_forward_angle: 75.0,
            start_wallrunning_up_impulse: 4.0,
            start_wallrunning_gravity_scale: 0.5,
            grounded_seconds_per_footstep: 1.0 / 4.0,
            wallrunning_seconds_per_footstep: 1.0 / 6.0,
            sliding_speed_factor: 0.8,
            sliding_max_forward_angle: 30.0,
            sliding_velocity_increase: [0.0, 0.0, -6.0],
            sliding_deceleration: [0.0, 0.0, 4.5],
            endless_slide_downhill_max_down_angle: 80.0,
            endless_slide_ground_normal_max_up_angle: 30.0,
            endless_sliding_acceleration: [0.0, 0.0, -10.0],
            event_queue_capacity: 10,
            default_boom_arm_length: 3.0,
            default_boom_arm_pitch_angle: 0.0,
            default_boom_arm_yaw_angle: 0.0,
            boom_lerp_factor: 0.9999,
            aim_boom_arm_length: 2.0,
            aim_boom_arm_pitch_angle: 0.0,
            aim_boom_arm_yaw_angle: 20.0,
            tpcombat_boom_rotation_lerp_factor: 0.9,
            rotate_body_to_movement_dir_lerp_factor: 0.999,
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
    pub fn max_standing_move_speed_continuous(&self) -> f32 {
        self.max_standing_move_speed_continuous
    }

    /// The max speed to which player's own forces can move its rigid body when crouched.
    pub fn max_crouched_move_speed_continuous(&self) -> f32 {
        self.max_crouched_move_speed_continuous
    }

    /// The max acceleration force the player can apply to its rigid body when standing.
    pub fn max_standing_move_acceleration_continuous(&self) -> f32 {
        self.max_standing_move_acceleration_continuous
    }

    /// The max acceleration force the player can apply to its rigid body when crouched.
    pub fn max_crouched_move_acceleration_continuous(&self) -> f32 {
        self.max_crouched_move_acceleration_continuous
    }

    /// The walk speed of the player when standing in the discrete movement mode.
    pub fn standing_walk_speed_discrete(&self) -> f32 {
        self.standing_walk_speed_discrete
    }

    /// The run speed of the player when standing in the discrete movement mode.
    pub fn standing_run_speed_discrete(&self) -> f32 {
        self.standing_run_speed_discrete
    }

    /// The sprint speed of the player when standing in the discrete movement mode.
    pub fn standing_sprint_speed_discrete(&self) -> f32 {
        self.standing_sprint_speed_discrete
    }

    /// The creep speed of the player when crouched in the discrete movement mode.
    pub fn crouched_creep_speed_discrete(&self) -> f32 {
        self.crouched_creep_speed_discrete
    }

    /// The walk acceleration force the player applies to its rigid body when standing in the discrete movement mode.
    pub fn standing_walk_acceleration_discrete(&self) -> f32 {
        self.standing_walk_acceleration_discrete
    }

    /// The run acceleration force the player applies to its rigid body when standing in the discrete movement mode.
    pub fn standing_run_acceleration_discrete(&self) -> f32 {
        self.standing_run_acceleration_discrete
    }

    /// The sprint acceleration force the player applies to its rigid body when standing in the discrete movement mode.
    pub fn standing_sprint_acceleration_discrete(&self) -> f32 {
        self.standing_sprint_acceleration_discrete
    }

    /// The creep acceleration force the player applies to its rigid body when crouched in the discrete movement mode.
    pub fn crouched_creep_acceleration_discrete(&self) -> f32 {
        self.crouched_creep_acceleration_discrete
    }

    /// The threshold (between 0 and 1) above which a player's movement vector's magnitude must be greater than to trigger a sprint.
    pub fn standing_sprint_input_threshold(&self) -> f32 {
        self.standing_sprint_input_threshold
    }

    /// The max angle between the forward vector and the movement vector under which the player can sprint in discrete movement mode.
    pub fn max_sprint_forward_angle_threshold_discrete(&self) -> f32 {
        self.max_sprint_forward_angle_threshold_discrete
    }

    pub fn discrete_movement_factor(&self) -> f32 {
        self.discrete_movement_factor
    }

    /// The threshold (between 0 and 1) above which a player's movement vector's magnitude must be greater than to trigger a run.
    pub fn standing_run_input_threshold(&self) -> f32 {
        self.standing_run_input_threshold
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

    /// The scale factor of jump force in the direction of the wall normal when wallrunning.
    pub fn jump_wallrunning_normal_scale(&self) -> f32 {
        self.jump_wallrunning_normal_scale
    }

    /// How close to straight down the body must be moving when wallrunning for the
    /// vertical velocity to be canceled before jumping off the wall.
    ///
    /// This is the minimum angle between the velocity and the down vector
    /// to be considered wallrunning downward.
    pub fn jump_wallrunning_down_velocity_angle_threshold(&self) -> f32 {
        self.jump_wallrunning_down_velocity_angle_threshold
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
    pub fn max_wallrunning_forward_angle(&self) -> f32 {
        self.max_wallrunning_forward_angle
    }

    /// The vertical acceleration applied to the player's
    /// body once wallrunning has started.
    pub fn start_wallrunning_up_impulse(&self) -> f32 {
        self.start_wallrunning_up_impulse
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
    pub fn sliding_max_forward_angle(&self) -> f32 {
        self.sliding_max_forward_angle
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

    /// The maximum angle between the player's velocity with
    /// the down vector (0, -1, 0) needed for the player to be
    /// considered traveling downhill.
    pub fn endless_slide_downhill_max_down_angle(&self) -> f32 {
        self.endless_slide_downhill_max_down_angle
    }

    /// The maximum dot factor of the player's ground normal
    /// with the up vector (0, 1, 0) to be considered traveling downhill.
    pub fn endless_slide_ground_normal_max_up_angle(&self) -> f32 {
        self.endless_slide_ground_normal_max_up_angle
    }

    /// The acceleration applied to endless / downhill slides.
    pub fn endless_sliding_acceleration(&self) -> [f32; 3] {
        self.endless_sliding_acceleration
    }

    /// The max capacity of the event channel used by the player structure.
    pub fn event_queue_capacity(&self) -> usize {
        self.event_queue_capacity
    }

    /// The length of the boom arm.
    pub fn default_boom_arm_length(&self) -> f32 {
        self.default_boom_arm_length
    }

    /// The pitch angle (about X axis) of the boom arm in degrees.
    pub fn default_boom_arm_pitch_angle(&self) -> f32 {
        self.default_boom_arm_pitch_angle
    }

    /// The yaw angle (about Y axis) of the boom arm in degrees.
    pub fn default_boom_arm_yaw_angle(&self) -> f32 {
        self.default_boom_arm_yaw_angle
    }

    /// How quickly the third person boom moves between the default and aim booms.
    pub fn boom_lerp_factor(&self) -> f32 {
        self.boom_lerp_factor
    }

    /// The length of the aiming boom arm.
    pub fn aim_boom_arm_length(&self) -> f32 {
        self.aim_boom_arm_length
    }

    /// The pitch angle (about X axis) of the aiming boom arm in degrees.
    pub fn aim_boom_arm_pitch_angle(&self) -> f32 {
        self.aim_boom_arm_pitch_angle
    }

    /// The yaw angle (about Y axis) of the aiming boom arm in degrees.
    pub fn aim_boom_arm_yaw_angle(&self) -> f32 {
        self.aim_boom_arm_yaw_angle
    }

    /// The lerp factor of player body isometry to the boom isometry
    /// while in third person combat mode.
    pub fn tpcombat_boom_rotation_lerp_factor(&self) -> f32 {
        self.tpcombat_boom_rotation_lerp_factor
    }

    /// The lerp factor for the player body to rotate in the player's movement direction.
    pub fn rotate_body_to_movement_dir_lerp_factor(&self) -> f32 {
        self.rotate_body_to_movement_dir_lerp_factor
    }
}
