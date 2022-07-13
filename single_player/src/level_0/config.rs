use perigee_core::{
    config::{PhysicsConfig, PlayerConfig},
    toml,
    traits::{TryFromToml, TryToToml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Level0Config {
    #[serde(default)]
    pub event_queue_capacity: Option<usize>,
    #[serde(default)]
    pub physics: PhysicsConfig,
    #[serde(default)]
    pub player: PlayerConfig,
}

impl Default for Level0Config {
    fn default() -> Self {
        Self {
            event_queue_capacity: Some(5),
            physics: PhysicsConfig::default(),
            player: PlayerConfig::default(),
        }
    }
}

impl TryFromToml for Level0Config {
    fn try_from_toml(toml_str: &str) -> Result<Self, String> {
        match toml::from_str::<Level0Config>(toml_str) {
            Ok(config) => Ok(config),
            Err(toml_de_err) => Err(toml_de_err.to_string()),
        }
    }
}

impl TryToToml for Level0Config {
    fn try_to_toml(&self) -> Result<String, String> {
        match toml::to_string(self) {
            Ok(config_toml) => Ok(config_toml),
            Err(toml_ser_err) => Err(toml_ser_err.to_string()),
        }
    }
}

impl Level0Config {
    pub fn event_queue_capacity(&self) -> Option<usize> {
        self.event_queue_capacity
    }

    pub fn physics(&self) -> PhysicsConfig {
        self.physics
    }

    pub fn player(&self) -> PlayerConfig {
        self.player
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_toml() {
        // Test normal conditions
        let config = Level0Config::try_from_toml(
            "event_queue_capacity = 5
        [physics]
        gravity = [0, -10.0, 0]
        event_queue_capacity = 7
        [player]
        max_look_up_angle = 1.5
        min_look_up_angle = -1.5
        max_standing_move_speed = 5.0
        max_crouched_move_speed = 2.0
        max_standing_move_acceleration = 150.0
        max_crouched_move_acceleration = 100.0
        capsule_standing_half_height = 0.5
        capsule_standing_radius = 0.5
        capsule_crouched_half_height = 0.2
        capsule_crouched_radius = 0.5
        jump_standing_acceleration = 500.0
        jump_crouched_acceleration = 400.0
        

        ",
        )
        .unwrap();

        assert_eq!(config.event_queue_capacity(), Some(5));
        assert_eq!(config.physics().gravity(), [0.0, -10.0, 0.0]);
        assert_eq!(config.physics().event_queue_capacity(), 7);
        assert_eq!(config.player().max_look_up_angle(), 1.5);
        assert_eq!(config.player().min_look_up_angle(), -1.5);
        assert_eq!(config.player().max_standing_move_speed(), 5.0);
        assert_eq!(config.player().max_crouched_move_speed(), 2.0);
        assert_eq!(config.player().max_standing_move_acceleration(), 150.0);
        assert_eq!(config.player().max_crouched_move_acceleration(), 100.0);
        assert_eq!(config.player().capsule_standing_half_height(), 0.5);
        assert_eq!(config.player().capsule_standing_radius(), 0.5);
        assert_eq!(config.player().capsule_crouched_half_height(), 0.2);
        assert_eq!(config.player().capsule_crouched_radius(), 0.5);
        assert_eq!(config.player().jump_standing_acceleration(), 500.0);
        assert_eq!(config.player().jump_crouched_acceleration(), 400.0);
    }

    #[test]
    fn try_to_toml() {
        // Test normal conditions
        let config = Level0Config::default();
        let config_toml = config.try_to_toml().unwrap();

        assert_eq!(config_toml, "event_queue_capacity = 5\n\n[physics]\ngravity = [0.0, -9.81, 0.0]\nevent_queue_capacity = 5\n\n[player]\nmass = 1.0\nmax_look_up_angle = 1.5707964\nmin_look_up_angle = -1.3089969\nenter_head_tilt_factor = 0.12\nexit_head_tilt_factor = 0.08\nnonstationary_speed_threshold = 0.01\nmax_standing_move_speed = 5.0\nmax_crouched_move_speed = 2.5\nmax_standing_move_acceleration = 25.0\nmax_crouched_move_acceleration = 12.5\ncapsule_standing_half_height = 0.5\ncapsule_standing_radius = 0.5\ncapsule_crouched_half_height = 0.2\ncapsule_crouched_radius = 0.5\nmax_jump_coyote_duration = 0.275\njump_standing_acceleration = 10.0\njump_crouched_acceleration = 3.5\nmin_jump_standing_cooldown_duration = 0.3\nmin_jump_crouched_cooldown_duration = 0.5\njump_wallrunning_scale = 1.0\nwallrunning_ray_length = 0.4\nground_ray_length = 0.1\nwallrunning_dot_value = 0.5\nstart_wallrunning_up_acceleration = 2.0\nstart_wallrunning_gravity_scale = 0.5\ngrounded_seconds_per_footstep = 0.25\nwallrunning_seconds_per_footstep = 0.16666667\n");
    }
}
