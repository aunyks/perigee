use crate::shared::{input::Input, player::Player, settings::GameSettings};
use crate::{level_0::config::Level0Config, shared::events::PlayerEvent};
use events::Level0Event;
use perigee::prelude::*;
use serde::{Deserialize, Serialize};

mod config;
mod events;

#[derive(Serialize, Deserialize)]
pub struct Sim<'a> {
    version: (u8, u8, u8),
    config: Level0Config,
    pub settings: GameSettings,
    pub physics: PhysicsWorld,
    pois: PointsOfInterest,
    pub player: Player,
    scene_gltf_bytes: &'a [u8],
    player_gltf_bytes: &'a [u8],
    #[serde(skip)]
    level_event_channel: EventChannel<Level0Event>,
    #[serde(skip)]
    pub input: Input,
}

impl<'a> Default for Sim<'a> {
    fn default() -> Self {
        let mut game = Self {
            version: (0, 0, 0),
            config: Level0Config::default(),
            settings: GameSettings::default(),
            input: Input::default(),
            physics: PhysicsWorld::default(),
            player: Player::default(),
            scene_gltf_bytes: include_bytes!("../../../assets/gltf/levels/0/scene.glb"),
            player_gltf_bytes: include_bytes!("../../../assets/gltf/shared/player-character.glb"),
            level_event_channel: EventChannel::default(),
            pois: PointsOfInterest::default(),
        };
        game.configure(None);
        game
    }
}

// Simple setup and accessors
impl<'a> Sim<'a> {
    pub fn with_config(config: Level0Config) -> Self {
        let mut game = Self::default();
        game.configure(Some(config));
        game
    }

    /// Get a copy of the game's current configuration.
    pub fn config(&self) -> Level0Config {
        self.config
    }

    /// Get a copy of the game's current settings.
    pub fn settings(&self) -> GameSettings {
        self.settings
    }

    /// Reconfigure this game using the provided configuration.
    pub fn configure(&mut self, new_config: Option<Level0Config>) {
        if let Some(config) = new_config {
            self.config = config;
        }
        self.physics = PhysicsWorld::with_config(self.config.physics());
        if let Some(queue_cap) = self.config.level_event_queue_capacity() {
            self.level_event_channel = EventChannel::with_capacity(queue_cap);
        }
    }

    pub fn get_level_event(&self) -> Result<Level0Event, TryRecvError> {
        self.level_event_channel.get_message()
    }

    pub fn scene_gltf_bytes(&self) -> &[u8] {
        self.scene_gltf_bytes
    }

    pub fn player_gltf_bytes(&self) -> &[u8] {
        self.player_gltf_bytes
    }
}

#[ffi]
impl<'a> Sim<'a> {
    #[ffi_only]
    pub fn scene_gltf_bytes_ptr(&self) -> *const u8 {
        self.scene_gltf_bytes().as_ptr()
    }

    #[ffi_only]
    pub fn scene_gltf_bytes_len(&self) -> usize {
        self.scene_gltf_bytes().len()
    }

    #[ffi_only]
    pub fn player_gltf_bytes_ptr(&self) -> *const u8 {
        self.player_gltf_bytes().as_ptr()
    }

    #[ffi_only]
    pub fn player_gltf_bytes_len(&self) -> usize {
        self.player_gltf_bytes().len()
    }

    #[slot_return]
    pub fn prop_isometry(&self, prop_name: &str) -> &Isometry<f32, UnitQuaternion<f32>, 3> {
        let prop_body_handle = self
            .physics
            .rigid_body_handle_with_name(prop_name)
            .expect("No prop has provided name.");
        self.physics
            .rigid_body_set
            .get(*prop_body_handle)
            .expect("Prop with provided name doesn't exist in physics world.")
            .position()
    }

    #[ffi_only]
    pub fn settings_left_right_look_sensitivity(&self) -> u8 {
        self.settings.left_right_look_sensitivity()
    }

    #[ffi_only]
    pub fn settings_up_down_look_sensitivity(&self) -> u8 {
        self.settings.up_down_look_sensitivity()
    }

    #[ffi_only]
    pub fn settings_set_left_right_look_sensitivity(&mut self, new_sensitivity: i32) {
        self.settings
            .set_left_right_look_sensitivity(new_sensitivity as u8);
    }

    #[ffi_only]
    pub fn settings_set_up_down_look_sensitivity(&mut self, new_sensitivity: i32) {
        self.settings
            .set_up_down_look_sensitivity(new_sensitivity as u8);
    }

    #[ffi_only]
    pub fn input_set_move_forward(&mut self, new_magnitude: f32) {
        self.input.set_move_forward(new_magnitude);
    }

    #[ffi_only]
    pub fn input_set_move_right(&mut self, new_magnitude: f32) {
        self.input.set_move_right(new_magnitude);
    }

    #[ffi_only]
    pub fn input_set_rotate_up(&mut self, new_magnitude: f32) {
        self.input.set_rotate_up(new_magnitude);
    }

    #[ffi_only]
    pub fn input_set_rotate_right(&mut self, new_magnitude: f32) {
        self.input.set_rotate_right(new_magnitude);
    }

    #[ffi_only]
    pub fn input_set_jump(&mut self, jump_val: u8) {
        self.input.set_jump(jump_val > 0)
    }

    #[ffi_only]
    pub fn input_set_crouch(&mut self, crouch_val: u8) {
        self.input.set_crouch(crouch_val > 0)
    }

    #[ffi_only]
    pub fn input_set_aim(&mut self, aim_val: u8) {
        self.input.set_aim(aim_val > 0)
    }

    #[slot_return]
    pub fn camera_global_isometry(&self) -> Isometry<f32, UnitQuaternion<f32>, 3> {
        // The player's head position
        self.player.camera_isometry()
    }

    #[slot_return]
    pub fn player_body_isometry(&self) -> Isometry<f32, UnitQuaternion<f32>, 3> {
        *self.player.body_isometry()
    }

    #[ffi_only]
    pub fn initialize_sim(&mut self) {
        // Load static colliders using trimeshes extracted from geometries
        // within a glTF. This lets you create a level using your favoritte 3D
        // modeling tool.
        let scene_gltf = Gltf::from_slice(self.scene_gltf_bytes).unwrap();

        self.physics.load_from_gltf(&scene_gltf).unwrap();
        self.pois.load_from_gltf(&scene_gltf).unwrap();

        let player_gltf = Gltf::from_slice(self.player_gltf_bytes).unwrap();
        self.player = Player::with_config(self.config().player());
        self.player.add_to_physics_world(
            &mut self.physics.rigid_body_set,
            &mut self.physics.collider_set,
            None,
        );
        self.player.add_gltf_animations(&player_gltf);
    }

    /// Step the game simulation by the provided number of seconds.
    pub fn step(&mut self, delta_seconds: f32) {
        self.player.update(
            delta_seconds,
            &mut self.input,
            &self.settings,
            &mut self.physics,
        );

        self.physics.step(delta_seconds);

        while let Ok(_collision_event) = self.physics.get_collider_event() {
            // match collision_event {
            //     CollisionEvent::Started(col1, col2, _) | CollisionEvent::Stopped(col1, col2, _) => {
            //         debug!("{:?} {:?} {:?}", self.player.collider_handle(), col1, col2);
            //     }
            // }
            // debug!(
            //     "{:?} {:?} {:?}",
            //     collision_event.sensor(),
            //     collision_event.started(),
            //     collision_event.stopped()
            // );
        }

        // Ease the pressure of this channel
        while let Ok(player_event) = self.player.get_event() {
            match player_event {
                PlayerEvent::Stepped => play_2d_audio("PLAYER_STEP"),
                PlayerEvent::Jump => play_2d_audio("PLAYER_JUMP"),
                PlayerEvent::StartedWallRunning => loop_2d_audio("PLAYER_WALLRUN"),
                PlayerEvent::StoppedWallRunning => stop_2d_audio("PLAYER_WALLRUN"),
                PlayerEvent::StartedSliding => loop_2d_audio("PLAYER_SLIDE"),
                PlayerEvent::StoppedSliding => stop_2d_audio("PLAYER_SLIDE"),
                _ => {}
            };
        }

        // Clear all inputs
        self.input = Input::default();
    }
}

#[cfg(feature = "ffi")]
#[no_mangle]
pub extern "C" fn destroy_sim(sim_ptr: *mut Sim) {
    // Box will deallocate the memory on drop
    unsafe { Box::from_raw(sim_ptr) };
}

#[cfg(feature = "ffi")]
#[no_mangle]
pub extern "C" fn create_sim() -> *mut Sim<'static> {
    init_perigee_logger();
    Box::into_raw(Box::new(Sim::default()))
}
