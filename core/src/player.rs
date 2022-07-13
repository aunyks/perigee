use crate::data_structures::{Queue, StateMachine};
use crate::events::{GameEvent, PlayerEvent};
use crate::interactions::InteractionGroup;
use crate::math::{move_towards, project_on_plane};
use crate::time::PassiveClock;
use crate::{config::PlayerConfig, input::Input, physics::PhysicsWorld, settings::GameSettings};
use rapier3d::{
    na::{Isometry, Quaternion, Unit, UnitQuaternion, Vector3},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const COLLIDER_RAYCAST_OFFSET: f32 = 0.001;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum CrouchState {
    Upright,
    Crouched,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum WallRunning {
    OnRight(Vector3<f32>),
    OnLeft(Vector3<f32>),
    None,
}

impl PartialEq for WallRunning {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WallRunning::OnLeft(_), WallRunning::OnLeft(_))
            | (WallRunning::OnRight(_), WallRunning::OnRight(_))
            | (WallRunning::None, WallRunning::None) => true,
            _ => false,
        }
    }
}

impl Eq for WallRunning {}

fn query_filter_excluding_player() -> QueryFilter<'static> {
    QueryFilter {
        groups: Some(
            InteractionGroups::all().with_filter(
                u32::from(InteractionGroup::All) ^ u32::from(InteractionGroup::Player),
            ),
        ),
        ..Default::default()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player<'a> {
    config: PlayerConfig,
    // Head up down rotation
    head_x_rotation: Unit<Quaternion<f32>>,
    // Head tilt rotation
    head_z_rotation: Unit<Quaternion<f32>>,
    head_isometry: Isometry<f32, Unit<Quaternion<f32>>, 3>,
    body_isometry: Isometry<f32, Unit<Quaternion<f32>>, 3>,
    rigid_body_handle: RigidBodyHandle,
    is_grounded: bool,
    wallrunning_state: StateMachine<WallRunning>,
    crouch_state: StateMachine<CrouchState>,
    #[serde(skip, default = "query_filter_excluding_player")]
    query_filter_excluding_player: QueryFilter<'a>,
    is_moving_non_vertically: bool,
    ground_normal: Vector3<f32>,
    footstep_timer: PassiveClock,
    coyote_timer: PassiveClock,
    jump_cooldown_timer: PassiveClock,
}

impl<'a> Default for Player<'a> {
    fn default() -> Self {
        let player_config = PlayerConfig::default();
        Self {
            config: player_config,
            head_x_rotation: UnitQuaternion::default(),
            head_z_rotation: UnitQuaternion::default(),
            head_isometry: Isometry::from(Vector3::from(
                player_config.standing_head_translation_offset(),
            )),
            body_isometry: Isometry::identity(),
            rigid_body_handle: RigidBodyHandle::from_raw_parts(0, 0),
            is_grounded: false,
            wallrunning_state: StateMachine::new(WallRunning::None),
            crouch_state: StateMachine::new(CrouchState::Upright),
            query_filter_excluding_player: query_filter_excluding_player(),
            is_moving_non_vertically: false,
            ground_normal: Vector::y(),
            footstep_timer: PassiveClock::default(),
            coyote_timer: PassiveClock::default(),
            jump_cooldown_timer: PassiveClock::default(),
        }
    }
}

impl<'a> Player<'a> {
    /// Create a new player with default properties.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new player with the provided configuration.
    pub fn with_config(config: PlayerConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    fn build_collider(&self, capsule_half_height: f32, capsule_radius: f32) -> Collider {
        ColliderBuilder::capsule_y(capsule_half_height, capsule_radius)
            .collision_groups(
                InteractionGroups::all().with_memberships(InteractionGroup::Player.into()),
            )
            // Listen for *all* collision and intersection events with
            // this collider
            // .active_events(ActiveEvents::COLLISION_EVENTS)
            // Set the mass (in kg, I think) of the collider
            .density(self.config.mass())
            .build()
    }

    /// Create a rigid body and collider for the player based on the the provided configuration parameters
    /// and / or default parameters, then add them to the provided `RigidBodySet` and `ColliderSet`.
    ///
    /// This should be called after creating the player and before updating the player.
    pub fn add_to_physics_world(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        initial_isometry: Option<Isometry<f32, Unit<Quaternion<f32>>, 3>>,
    ) {
        let initial_isometry = if let Some(initial_isometry) = initial_isometry {
            initial_isometry
        } else {
            Isometry::from_parts(
                Translation::new(0.0, 1.0, 0.0),
                UnitQuaternion::from_quaternion(Quaternion::identity()),
            )
        };
        let rigid_body = RigidBodyBuilder::dynamic()
            .position(initial_isometry)
            // Don't tip over
            .enabled_rotations(false, true, false)
            // Don't ever rotate without being explicitly set
            .angular_damping(1000.0)
            .build();
        let (capsule_half_height, capsule_radius) = match self.crouch_state.current_state() {
            CrouchState::Upright => (
                self.config.capsule_standing_half_height(),
                self.config.capsule_standing_radius(),
            ),
            CrouchState::Crouched => (
                self.config.capsule_crouched_half_height(),
                self.config.capsule_crouched_radius(),
            ),
        };
        let collider = self.build_collider(capsule_half_height, capsule_radius);

        let player_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, player_body_handle, rigid_body_set);
        self.set_body_handle(player_body_handle);
    }

    /// Update the player based on what it knows about its internal properties
    /// and the properties of its rigid body. This should be called after the [Input](crate::input::Input)
    /// and [GameSettings](crate::settings::GameSettings) are updated but before the [PhysicsWorld](crate::physics::PhysicsWorld)
    /// steps and the `Player`'s events are extracted this frame.
    pub fn update<T>(
        &mut self,
        delta_seconds: f32,
        input: &mut Input,
        settings: &GameSettings,
        physics: &mut PhysicsWorld,
        game_events: &mut Queue<GameEvent<T>>,
    ) where
        u32: From<T>,
    {
        self.update_body_isometry(&mut physics.rigid_body_set);
        self.update_head_isometry();

        self.rotate_body(
            -input.rotate_right()
                * (2.5 * f32::from(settings.left_right_look_sensitivity() / 5)).to_radians(),
            &mut physics.rigid_body_set,
        );
        self.rotate_head(
            input.rotate_up()
                * (5.0 * f32::from(settings.up_down_look_sensitivity() / 5)).to_radians(),
            self.config.max_look_up_angle(),
            self.config.min_look_up_angle(),
        );

        let (player_radius, player_half_height) = match self.crouch_state.current_state() {
            CrouchState::Upright => (
                self.config.capsule_standing_radius(),
                self.config.capsule_standing_total_height() / 2.0,
            ),
            CrouchState::Crouched => (
                self.config.capsule_crouched_radius(),
                self.config.capsule_crouched_total_height() / 2.0,
            ),
        };

        let previous_tick_grounded_state = self.is_grounded;
        self.determine_grounded_states(
            &mut physics.rigid_body_set,
            player_half_height,
            &mut physics.query_pipeline,
            &mut physics.collider_set,
        );
        let previous_tick_wallrunning_state = *self.wallrunning_state.current_state();
        self.determine_wallrunning_state(
            &mut physics.rigid_body_set,
            player_radius,
            &mut physics.query_pipeline,
            &mut physics.collider_set,
        );
        let previous_tick_is_moving_non_vertically = self.is_moving_non_vertically;
        self.determine_non_vertical_motion(&mut physics.rigid_body_set);

        if previous_tick_grounded_state != self.is_grounded {
            // We've just landed
            if self.is_grounded {
                // Sometimes if we're trying to jump as soon as we land,
                // the upward movement is canceled by gravity, resulting in a jump
                // that doesn't move vertically. Zeroing out the vertical velocity
                // fixes that and lets us jump up immediately again
                self.nullify_vertical_movement(&mut physics.rigid_body_set);
                // We can't be grounded and wallrunning at the same time
                self.wallrunning_state.transition_to(WallRunning::None);
                game_events.enqueue(PlayerEvent::Landed.into());
                self.coyote_timer.reset();
            } else {
                // We've just taken off
            }
        }
        if previous_tick_wallrunning_state != *self.wallrunning_state.current_state() {
            // We're entered a new wallrun
            if self.wallrunning_state != WallRunning::None && !self.is_grounded {
                self.start_wallrunning(&mut physics.rigid_body_set);
                game_events.enqueue(PlayerEvent::StartedWallRunning.into());
                self.coyote_timer.reset();
            } else {
                // We've exited a wallrun
                self.stop_wallrunning(&mut physics.rigid_body_set);
                game_events.enqueue(PlayerEvent::StoppedWallRunning.into());
            }
        }
        self.tilt_head();

        if self.is_grounded {
            match (
                previous_tick_is_moving_non_vertically,
                self.is_moving_non_vertically,
            ) {
                (true, false) => {
                    game_events.enqueue(PlayerEvent::Stopped.into());
                }
                (false, true) => {
                    game_events.enqueue(PlayerEvent::Moving.into());
                }
                _ => {}
            };
        }

        self.jump_cooldown_timer.tick(delta_seconds);
        let max_jump_cooldown_timer_duration = match self.crouch_state.current_state() {
            CrouchState::Upright => self.config.min_jump_standing_cooldown_duration(),
            CrouchState::Crouched => self.config.min_jump_crouched_cooldown_duration(),
        };
        if input.jump() {
            let jump_has_cooled_down = self.jump_cooldown_timer.elapsed()
                > Duration::from_secs_f32(max_jump_cooldown_timer_duration);
            let is_grounded_or_wallrunning =
                self.wallrunning_state != WallRunning::None || self.is_grounded;
            let can_coyote_jump = self.coyote_timer.elapsed()
                < Duration::from_secs_f32(self.config.max_jump_coyote_duration());

            if jump_has_cooled_down && (is_grounded_or_wallrunning || can_coyote_jump) {
                self.jump(&mut physics.rigid_body_set, game_events);
            }
        }
        if !self.is_grounded && self.wallrunning_state == WallRunning::None {
            self.coyote_timer.tick(delta_seconds);
        }

        if self.is_grounded {
            let max_move_speed = match self.crouch_state.current_state() {
                CrouchState::Upright => self.config.max_standing_move_speed(),
                CrouchState::Crouched => self.config.max_crouched_move_speed(),
            };
            let max_move_acceleration = match self.crouch_state.current_state() {
                CrouchState::Upright => self.config.max_standing_move_acceleration(),
                CrouchState::Crouched => self.config.max_crouched_move_acceleration(),
            };
            self.move_body(
                delta_seconds,
                input.move_right(),
                input.move_forward(),
                max_move_acceleration,
                max_move_speed,
                &mut physics.rigid_body_set,
            );
        }

        match (input.crouch(), self.crouch_state.current_state()) {
            (true, &CrouchState::Upright) => {
                self.change_crouch_state(
                    self.config.capsule_crouched_half_height(),
                    self.config.capsule_crouched_radius(),
                    &mut physics.rigid_body_set,
                    &mut physics.collider_set,
                    &mut physics.island_manager,
                );
                game_events.enqueue(PlayerEvent::Crouched.into());
            }
            (false, &CrouchState::Crouched) => {
                if self.can_stand_up(
                    &mut physics.rigid_body_set,
                    &mut physics.query_pipeline,
                    &mut physics.collider_set,
                ) {
                    self.change_crouch_state(
                        self.config.capsule_standing_half_height(),
                        self.config.capsule_standing_radius(),
                        &mut physics.rigid_body_set,
                        &mut physics.collider_set,
                        &mut physics.island_manager,
                    );
                    game_events.enqueue(PlayerEvent::StoodUpright.into());
                }
            }
            _ => {}
        }

        self.footstep_timer.tick(delta_seconds);
        // If we're wallrunning or on the ground, we should
        // make a note of when a step was taken. The frequency of
        // steps is a function of our speed.
        if self.is_grounded || self.wallrunning_state != WallRunning::None {
            let current_speed = self.velocity(&mut physics.rigid_body_set).magnitude();
            if current_speed > 0.0 {
                let max_possible_move_speed = self.config.max_standing_move_speed();
                let speed_ratio = max_possible_move_speed / current_speed;

                let secs_per_footstep = match self.wallrunning_state.current_state() {
                    WallRunning::None => self.config.grounded_seconds_per_footstep(),
                    _ => self.config.wallrunning_seconds_per_footstep(),
                };

                if self.footstep_timer.elapsed()
                    > Duration::from_secs_f32(secs_per_footstep * speed_ratio)
                {
                    game_events.enqueue(PlayerEvent::Stepped.into());
                    self.footstep_timer.reset();
                }
            }
        }
    }

    fn update_body_isometry(&mut self, rigid_body_set: &mut RigidBodySet) {
        let mut body_isometry: Isometry<f32, Unit<Quaternion<f32>>, 3> = Isometry::identity();
        let body_handle = self.body_handle();
        if let Some(body) = rigid_body_set.get_mut(body_handle) {
            body_isometry = *body.position();
        }
        self.body_isometry = body_isometry;
    }

    fn update_head_isometry(&mut self) {
        let translational_offset: [f32; 3] = match self.crouch_state.current_state() {
            CrouchState::Upright => self.config.standing_head_translation_offset(),
            CrouchState::Crouched => self.config.crouched_head_translation_offset(),
        };
        self.head_isometry = self.head_isometry.lerp_slerp(
            &Isometry::from_parts(
                translational_offset.into(),
                self.head_x_rotation * self.head_z_rotation,
            ),
            self.config.head_crouch_lerp_factor(),
        );
    }

    fn set_body_handle(&mut self, body_handle: RigidBodyHandle) {
        self.rigid_body_handle = body_handle;
    }

    pub fn body_handle(&self) -> RigidBodyHandle {
        self.rigid_body_handle
    }

    pub fn head_isometry(&self) -> Isometry<f32, Unit<Quaternion<f32>>, 3> {
        self.head_isometry
    }

    /// Get the isometry (position and orientation) of the player's rigid body.
    pub fn body_isometry(&self) -> Isometry<f32, Unit<Quaternion<f32>>, 3> {
        self.body_isometry
    }

    /// Rotate the player's rigid body about the Y axis (left / right) based on user input.
    pub fn rotate_body(&self, y_axis_rotation: f32, rigid_body_set: &mut RigidBodySet) {
        let body_handle = self.body_handle();
        if let Some(body) = rigid_body_set.get_mut(body_handle) {
            let new_body_rotation = body
                .position()
                .rotation
                .append_axisangle_linearized(&vector![0.0, y_axis_rotation, 0.0]);
            body.set_position(
                Isometry::from_parts(body.position().translation, new_body_rotation),
                false,
            );
        }
    }

    /// Rotate the player head about the X axis (up / down) based on user input, not exceeding the min or max look angles.
    pub fn rotate_head(&mut self, x_axis_rotation: f32, min_look_angle: f32, max_look_angle: f32) {
        let (roll, pitch, yaw) = self.head_x_rotation.euler_angles();

        self.head_x_rotation = UnitQuaternion::from_euler_angles(
            (roll + x_axis_rotation).clamp(max_look_angle, min_look_angle),
            pitch,
            yaw,
        );
    }

    /// Move the player rigid body laterally (in the X-Z direction) based on user input.
    pub fn move_body(
        &mut self,
        delta_seconds: f32,
        left_right_magnitude: f32,
        forward_back_magnitude: f32,
        max_move_acceleration: f32,
        max_speed: f32,
        rigid_body_set: &mut RigidBodySet,
    ) {
        let movement_vector = vector![left_right_magnitude, 0.0, forward_back_magnitude];
        let trying_to_move = movement_vector.magnitude() != 0.0;
        let max_velocity: Vector<f32> = if trying_to_move {
            // If we don't have this check, we'd be dividing 0 by 0 and
            // have a vector of NaNs
            movement_vector.cap_magnitude(1.0) * max_speed
        } else {
            movement_vector * max_speed
        };

        let body_handle = self.body_handle();
        if let Some(body) = rigid_body_set.get_mut(body_handle) {
            // Note: The two value is already transformed by the body
            // isometry
            let current_velocity: Vector<f32> = *body.linvel();

            // The max velocity transformed by the isometry (position & orientation)
            // of the player.
            let transformed_max_velocity = body.position().transform_vector(&max_velocity);
            // The player isometry-transformed max velocity rotated to point in the
            // direction of the slope the player is currently on
            let vertical_transformed_max_velocity =
                project_on_plane(&transformed_max_velocity, &self.ground_normal);

            let goal_velocity = move_towards(
                &vertical_transformed_max_velocity,
                &current_velocity,
                max_move_acceleration * delta_seconds,
            );

            let acceleration = ((goal_velocity - current_velocity) / delta_seconds)
                .cap_magnitude(max_move_acceleration);

            body.reset_forces(true);
            body.add_force(acceleration * body.mass(), true);
        }
    }

    fn jump<T>(&mut self, rigid_body_set: &mut RigidBodySet, game_events: &mut Queue<GameEvent<T>>)
    where
        u32: From<T>,
    {
        let jump_acceleration = match self.crouch_state.current_state() {
            CrouchState::Upright => self.config.jump_standing_acceleration(),
            CrouchState::Crouched => self.config.jump_crouched_acceleration(),
        };
        self.jump_body(jump_acceleration, rigid_body_set);
        game_events.enqueue(PlayerEvent::Jump.into());
        self.jump_cooldown_timer.reset();
    }

    /// Make the player's rigid body jump. If the player is wallrunning, it will jump on the
    /// opposite direction of the wall it's running on. If not wallrunning, it will jump straight up.
    pub fn jump_body(&mut self, jump_acceleration: f32, rigid_body_set: &mut RigidBodySet) {
        let body_handle = self.body_handle();
        let body_isometry = self.body_isometry();
        // We always wanna jump up and forward
        let untransformed_jump_direction_vector = vector![0.0, 2.0, -2.0];
        // And also away from the wall
        let jump_vector = match self.wallrunning_state.current_state() {
            WallRunning::OnRight(wall_normal) => {
                (wall_normal + untransformed_jump_direction_vector)
                    * self.config.jump_wallrunning_scale()
            }
            WallRunning::OnLeft(wall_normal) => {
                (wall_normal + untransformed_jump_direction_vector)
                    * self.config.jump_wallrunning_scale()
            }
            WallRunning::None => vector![0.0, 1.0, 0.0],
        } * jump_acceleration;
        if let Some(body) = rigid_body_set.get_mut(body_handle) {
            let transformed_jump_vector = body_isometry.transform_vector(&jump_vector);
            let jump_impulse = body.mass() * transformed_jump_vector;
            body.apply_impulse(jump_impulse, true);
        }
    }

    fn determine_non_vertical_motion(&mut self, rigid_body_set: &mut RigidBodySet) {
        let body_handle = self.body_handle();
        if let Some(body) = rigid_body_set.get_mut(body_handle) {
            let linear_velocity = body.linvel();
            if vector![linear_velocity.x, 0.0, linear_velocity.z].magnitude()
                > self.config.nonstationary_speed_threshold()
            {
                self.is_moving_non_vertically = true;
                return;
            }
        }
        self.is_moving_non_vertically = false;
    }

    /// Set the body's vertical velocity to 0.
    fn nullify_vertical_movement(&self, rigid_body_set: &mut RigidBodySet) {
        let body_handle = self.body_handle();
        if let Some(body) = rigid_body_set.get_mut(body_handle) {
            let mut linvel = *body.linvel();
            linvel.y = 0.0;
            body.set_linvel(linvel, true);
        }
    }

    /// Determine whether the player has a collider just below it, functioning
    /// as ground. Also calculate the normal of this surface.
    fn determine_grounded_states(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        player_half_height: f32,
        query_pipeline: &mut QueryPipeline,
        collider_set: &mut ColliderSet,
    ) {
        let body_handle = self.body_handle();
        let body_isometry = self.body_isometry();
        if rigid_body_set.get_mut(body_handle).is_some() {
            let grounded_ray = Ray::new(point![0.0, 0.0, 0.0], vector![0.0, -1.0, 0.0])
                .transform_by(&body_isometry);
            if let Some((_, ray_intersection)) = query_pipeline.cast_ray_and_get_normal(
                &rigid_body_set,
                collider_set,
                &grounded_ray,
                player_half_height + COLLIDER_RAYCAST_OFFSET + self.config.ground_ray_length(),
                false,
                self.query_filter_excluding_player,
            ) {
                self.ground_normal = ray_intersection.normal;
                self.is_grounded = true;
                return;
            }
        }
        self.is_grounded = false;
    }

    /// Determine whether the player has a collider on the right or left side by firing a ray in those directions.
    ///
    /// Note: This will update the state *regardless* of whether the player is grounded, so you must ensure by
    /// yourself that the player isn't already grounded when responding to this state.
    fn determine_wallrunning_state(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        player_radius: f32,
        query_pipeline: &mut QueryPipeline,
        collider_set: &mut ColliderSet,
    ) {
        let body_handle = self.body_handle();
        let body_isometry = self.body_isometry();
        let ray_distance_from_body = self.config.wallrunning_ray_length();
        if let Some(body) = rigid_body_set.get_mut(body_handle) {
            // Can only wallrun if moving forward enough
            let transformed_forward_vector = self
                .body_isometry()
                .transform_vector(&vector![0.0, 0.0, -1.0]);
            if body.linvel().dot(&transformed_forward_vector) <= self.config.wallrunning_dot_value()
            {
                self.wallrunning_state.transition_to(WallRunning::None);
                return;
            }

            let right_wall_ray = Ray::new(point![0.0, 0.0, 0.0], vector![1.0, 0.0, 0.0])
                .transform_by(&body_isometry);

            if let Some((_handle, ray_intersection)) = query_pipeline.cast_ray_and_get_normal(
                &rigid_body_set,
                collider_set,
                &right_wall_ray,
                (player_radius - COLLIDER_RAYCAST_OFFSET) + ray_distance_from_body,
                false,
                self.query_filter_excluding_player,
            ) {
                let ray_normal = ray_intersection.normal;
                let untransformed_right_wall_ray_dir =
                    body_isometry.inverse_transform_vector(&right_wall_ray.dir);
                let right_wall_ray_dir_mirrored_x = body_isometry.transform_vector(&vector![
                    -untransformed_right_wall_ray_dir.x,
                    untransformed_right_wall_ray_dir.y,
                    untransformed_right_wall_ray_dir.z
                ]);
                let transformed_wall_normal =
                    (right_wall_ray_dir_mirrored_x + ray_normal).normalize();
                let wall_normal = body_isometry.inverse_transform_vector(&transformed_wall_normal);
                self.wallrunning_state
                    .transition_to(WallRunning::OnRight(wall_normal));
                return;
            }

            let left_wall_ray = Ray::new(point![0.0, 0.0, 0.0], vector![-1.0, 0.0, 0.0])
                .transform_by(&body_isometry);
            if let Some((_handle, ray_intersection)) = query_pipeline.cast_ray_and_get_normal(
                &rigid_body_set,
                collider_set,
                &left_wall_ray,
                (player_radius - COLLIDER_RAYCAST_OFFSET) + ray_distance_from_body,
                false,
                self.query_filter_excluding_player,
            ) {
                let ray_normal = ray_intersection.normal;
                let untransformed_left_wall_ray_dir =
                    body_isometry.inverse_transform_vector(&left_wall_ray.dir);
                let left_wall_ray_dir_mirrored_x = body_isometry.transform_vector(&vector![
                    -untransformed_left_wall_ray_dir.x,
                    untransformed_left_wall_ray_dir.y,
                    untransformed_left_wall_ray_dir.z
                ]);
                let transformed_wall_normal =
                    (left_wall_ray_dir_mirrored_x + ray_normal).normalize();
                let wall_normal = body_isometry.inverse_transform_vector(&transformed_wall_normal);
                self.wallrunning_state
                    .transition_to(WallRunning::OnLeft(wall_normal));
                return;
            }
        }
        self.wallrunning_state.transition_to(WallRunning::None);
    }

    /// Tilt the head of the player about the Z axis based on the current wall running state.
    /// If the player is on a wall on the right, tilt the head left. If the wall is on the left, tilt
    /// the head right. If not wall running, don't tilt the head.
    fn tilt_head(&mut self) {
        let z_axis = Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0));
        let max_tilt = 10.0f32.to_radians();
        let target_head_z_rotation = if !self.is_grounded {
            match self.wallrunning_state.current_state() {
                WallRunning::OnRight(_) => UnitQuaternion::from_axis_angle(&z_axis, max_tilt),
                WallRunning::OnLeft(_) => UnitQuaternion::from_axis_angle(&z_axis, -max_tilt),
                WallRunning::None => UnitQuaternion::from_axis_angle(&z_axis, 0.0),
            }
        } else {
            UnitQuaternion::from_axis_angle(&z_axis, 0.0)
        };
        let tilt_speed = if self.wallrunning_state == WallRunning::None {
            self.config.enter_head_tilt_factor()
        } else {
            self.config.exit_head_tilt_factor()
        };
        self.head_z_rotation = self
            .head_z_rotation
            .try_slerp(&target_head_z_rotation, tilt_speed, 0.0).expect("Could not tilt player head as found and desired quaternions were 180 degrees apart");
    }

    /// Determine whether the player can stand up by casting a ray straight above the head.
    fn can_stand_up(
        &self,
        rigid_body_set: &mut RigidBodySet,
        query_pipeline: &mut QueryPipeline,
        collider_set: &mut ColliderSet,
    ) -> bool {
        let body_handle = self.body_handle();
        let body_isometry = self.body_isometry();
        if rigid_body_set.get_mut(body_handle).is_some() {
            // Make sure there's enough space for the standing collider to fit.
            // If so, we can stand. If not, we can't.
            let above_head_ray = Ray::new(
                point![0.0, self.config.capsule_crouched_total_height() / 2.0, 0.0],
                vector![0.0, 1.0, 0.0],
            )
            .transform_by(&body_isometry);

            let distance_between_standing_and_crouched_heights =
                self.config.capsule_standing_total_height()
                    - self.config.capsule_crouched_total_height();
            if query_pipeline
                .cast_ray(
                    &rigid_body_set,
                    collider_set,
                    &above_head_ray,
                    COLLIDER_RAYCAST_OFFSET + distance_between_standing_and_crouched_heights,
                    false,
                    self.query_filter_excluding_player,
                )
                .is_some()
            {
                return false;
            }
        }
        true
    }

    /// Change the size of the player collider.
    fn change_crouch_state(
        &mut self,
        new_capsule_half_height: f32,
        new_capsule_radius: f32,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        island_manager: &mut IslandManager,
    ) {
        if let Some(body) = rigid_body_set.get_mut(self.body_handle()) {
            // Toggle crouch state
            self.crouch_state
                .transition_to(match self.crouch_state.current_state() {
                    CrouchState::Upright => CrouchState::Crouched,
                    CrouchState::Crouched => {
                        // // Prevent any intersections between the larger collider and the ground
                        // let mut new_pos = self.body_isometry();
                        // new_pos.translation.y += (self.config.capsule_standing_half_height()
                        //     + self.config.capsule_standing_radius())
                        //     - (self.config.capsule_crouched_half_height()
                        //         + self.config.capsule_crouched_radius());
                        // body.set_position(new_pos, true);
                        // // Or something like this
                        // body.add_force(vector![0.0, 125.0, 0.0], true);
                        CrouchState::Upright
                    }
                });
            let body_collider_handle = body.colliders()[0];
            let new_collider = self.build_collider(new_capsule_half_height, new_capsule_radius);
            collider_set.remove(body_collider_handle, island_manager, rigid_body_set, true);
            collider_set.insert_with_parent(new_collider, self.body_handle(), rigid_body_set);
        }
    }

    fn start_wallrunning(&mut self, rigid_body_set: &mut RigidBodySet) {
        if let Some(body) = rigid_body_set.get_mut(self.body_handle()) {
            body.reset_forces(true);
            body.set_gravity_scale(self.config.start_wallrunning_gravity_scale(), true);
            let current_linvel = body.linvel();
            let new_linvel = vector![current_linvel.x, 0.0, current_linvel.z];
            body.set_linvel(new_linvel, true);
            body.apply_impulse(
                vector![
                    0.0,
                    self.config.start_wallrunning_up_acceleration() * body.mass(),
                    0.0
                ] * body.mass(),
                true,
            );
        }
    }

    fn stop_wallrunning(&mut self, rigid_body_set: &mut RigidBodySet) {
        if let Some(body) = rigid_body_set.get_mut(self.body_handle()) {
            body.set_gravity_scale(1.0, true);
        }
    }

    fn velocity(&self, rigid_body_set: &mut RigidBodySet) -> Vector3<f32> {
        if let Some(body) = rigid_body_set.get(self.body_handle()) {
            return *body.linvel();
        }
        vector![0.0, 0.0, 0.0]
    }
}
