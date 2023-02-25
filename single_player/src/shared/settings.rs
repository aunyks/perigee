use perigee::{
    toml,
    traits::{TryFromToml, TryToToml},
};
use serde::{Deserialize, Serialize};

// The player-editable game configuration.
/// These should be editable at runtime.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct GameSettings {
    #[serde(default)]
    up_down_look_sensitivity: u8,
    #[serde(default)]
    left_right_look_sensitivity: u8,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            up_down_look_sensitivity: 5,
            left_right_look_sensitivity: 5,
        }
    }
}

impl TryFromToml for GameSettings {
    fn try_from_toml(toml_str: &str) -> Result<Self, String> {
        match toml::from_str::<GameSettings>(toml_str) {
            Ok(settings) => Ok(settings),
            Err(toml_de_err) => Err(toml_de_err.to_string()),
        }
    }
}

impl TryToToml for GameSettings {
    fn try_to_toml(&self) -> Result<String, String> {
        match toml::to_string(self) {
            Ok(settings_toml) => Ok(settings_toml),
            Err(toml_ser_err) => Err(toml_ser_err.to_string()),
        }
    }
}

impl GameSettings {
    /// How quickly the player can change its X axis (up / down)
    /// look sensitivity.
    pub fn up_down_look_sensitivity(&self) -> u8 {
        self.up_down_look_sensitivity
    }

    /// How quickly the player can change its Y axis (left / right)
    /// look sensitivity.
    pub fn left_right_look_sensitivity(&self) -> u8 {
        self.left_right_look_sensitivity
    }

    /// Set how quickly the player can change its X axis (up / down)
    /// look sensitivity.
    pub fn set_up_down_look_sensitivity(&mut self, new_sensitivity: u8) {
        self.up_down_look_sensitivity = new_sensitivity;
    }

    /// Set how quickly the player can change its Y axis (left / right)
    /// look sensitivity.
    pub fn set_left_right_look_sensitivity(&mut self, new_sensitivity: u8) {
        self.left_right_look_sensitivity = new_sensitivity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_toml() {
        // Test normal conditions
        let settings = GameSettings::try_from_toml(
            "
        up_down_look_sensitivity = 7
        left_right_look_sensitivity = 3
        ",
        )
        .unwrap();

        assert_eq!(settings.up_down_look_sensitivity, 7);
        assert_eq!(settings.left_right_look_sensitivity, 3);
    }

    #[test]
    fn try_to_toml() {
        // Test normal conditions
        let settings = GameSettings::default();
        let settings_toml = settings.try_to_toml().unwrap();

        assert_eq!(
            settings_toml,
            "up_down_look_sensitivity = 5\nleft_right_look_sensitivity = 5\n"
        );
    }
}
