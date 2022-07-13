use crate::level_0::config::Level0Config;
pub use perigee_core;
use perigee_core::{
    audiovisual_assets::{Animation, Asset},
    data_structures::Queue,
    events::{audiovisual::AudioVisualOperation, AudioVisualEvent, GameEvent},
    gltf::Gltf,
    input::Input,
    physics::PhysicsWorld,
    player::Player,
    settings::GameSettings,
};
use serde::{Deserialize, Serialize};
mod config;

#[derive(Serialize, Deserialize)]
pub struct Level1Event;
impl From<Level1Event> for u32 {
    fn from(_: Level1Event) -> Self {
        0
    }
}

// Note: #[serde(default)] means that if the
//       field isn't defined during deserialization
//       then its `Default` value is used.
//
//       #[serde(skip)] means that we skip the field
//       during serialization and don't include it in the output.
#[derive(Serialize, Deserialize)]
pub struct Sim<'a> {
    version: (u8, u8, u8),
    config: Level0Config,
    pub settings: GameSettings,
    #[serde(skip)]
    pub input: Input,
    physics: PhysicsWorld,
    pub player: Player<'a>,
    pub events: Queue<GameEvent<Level1Event>>,
}
unsafe impl<'a> Send for Sim<'a> {}
unsafe impl<'a> Sync for Sim<'a> {}

impl<'a> Default for Sim<'a> {
    fn default() -> Self {
        let mut game = Self {
            version: (0, 0, 0),
            config: Level0Config::default(),
            settings: GameSettings::default(),
            input: Input::default(),
            physics: PhysicsWorld::default(),
            player: Player::default(),
            events: Queue::default(),
        };
        game.configure_self();
        game
    }
}

// Simple setup and accessors
impl<'a> Sim<'a> {
    /// Create a new game using default parameters and configuration.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: Level0Config) -> Self {
        let mut game = Self::default();
        game.configure(config);
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

    /// Configure this game using its current configuration.
    ///
    /// You should prefer using the [configure()](crate::Sim::configure()) function
    /// over this one.
    pub fn configure_self(&mut self) {
        self.physics = PhysicsWorld::with_config(self.config.physics());
        if let Some(queue_cap) = self.config.event_queue_capacity() {
            self.events = Queue::with_capacity(queue_cap);
        }
    }

    /// Reconfigure this game using the provided configuration.
    pub fn configure(&mut self, new_config: Level0Config) {
        self.config = new_config;
        self.configure_self();
    }
}

impl<'a> Sim<'a> {
    /// Set up the game, creating entities that define the simulated scene.
    pub fn initialize(&mut self) -> Result<(), String> {
        // Load static colliders using trimeshes extracted from geometries
        // within a glTF. This lets you create a level using your favoritte 3D
        // modeling tool.
        let static_gltf = match Gltf::from_slice(include_bytes!(
            "../../../assets/gltf/levels/0/physics-world.glb"
        )) {
            Ok(gltf) => gltf,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        // Use the returned handles to remove these meshes from the physics world later on
        let _static_trimesh_handles = self
            .physics
            .add_static_objects_from_gltf(&static_gltf)
            .unwrap();

        /* Create the ground. */
        // let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
        // let ground_collider_handle = self.physics.collider_set.insert(collider);

        self.player = Player::with_config(self.config().player());
        self.player.add_to_physics_world(
            &mut self.physics.rigid_body_set,
            &mut self.physics.collider_set,
            None,
        );

        self.events
            .enqueue(GameEvent::AudioVisual(AudioVisualEvent::new(
                AudioVisualOperation::Loop,
                Asset::Animation(Animation::CameraIdle),
            )));

        Ok(())
    }

    /// Step the game simulation by the provided number of seconds.
    pub fn step(&mut self, delta_seconds: f32) {
        self.player.update(
            delta_seconds,
            &mut self.input,
            &self.settings,
            &mut self.physics,
            &mut self.events,
        );

        self.physics.step(delta_seconds);

        // Empty the collision event queue. For now,
        // this is just to prevent any backpressure in
        // the queue that could cause it to do allocations we don't want.
        while !self.physics.event_queue.is_empty() {
            let _ = self.physics.event_queue.dequeue();
        }

        // Clear all inputs
        self.input = Input::default();
    }
}
