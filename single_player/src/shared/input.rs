use perigee::{
    toml,
    traits::{TryFromToml, TryToToml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Input {
    /// The forward moving magnitude of the object
    /// controlled by the player (back is positive, forward is negative)
    move_forward: f32,
    /// The right moving magnitude of the object
    /// controlled by the player (right is positive, left is negative)
    move_right: f32,
    /// The look-up magnitude of the object
    /// controlled by the player (up is positive, down is negative)
    rotate_up: f32,
    /// The right turn magnitude of the object
    /// controlled by the player (right is positive, left is negative)
    rotate_right: f32,
    /// The jump status of the object controlled
    /// by the player (true is intention to jump, false is not)
    jump: bool,
    /// The crouch status of the player (true is intention to crouch, false is not)
    crouch: bool,
    /// The third person aim mode of the player
    aim: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            move_forward: 0.0,
            move_right: 0.0,
            rotate_up: 0.0,
            rotate_right: 0.0,
            jump: false,
            crouch: false,
            aim: false,
        }
    }
}

impl TryFromToml for Input {
    fn try_from_toml(toml_str: &str) -> Result<Self, String> {
        match toml::from_str::<Input>(toml_str) {
            Ok(input) => Ok(input),
            Err(toml_de_err) => Err(toml_de_err.to_string()),
        }
    }
}

impl TryToToml for Input {
    fn try_to_toml(&self) -> Result<String, String> {
        match toml::to_string(self) {
            Ok(input_toml) => Ok(input_toml),
            Err(toml_ser_err) => Err(toml_ser_err.to_string()),
        }
    }
}

impl Input {
    /// The forward moving magnitude of the object
    /// controlled by the player (back is positive, forward is negative).
    pub fn move_forward(&self) -> f32 {
        self.move_forward
    }

    /// The right moving magnitude of the object
    /// controlled by the player (right is positive, left is negative).
    pub fn move_right(&self) -> f32 {
        self.move_right
    }

    /// The look-up magnitude of the object
    /// controlled by the player (up is positive, down is negative).
    pub fn rotate_up(&self) -> f32 {
        self.rotate_up
    }

    /// The right turn magnitude of the object
    /// controlled by the player (right is positive, left is negative).
    pub fn rotate_right(&self) -> f32 {
        self.rotate_right
    }

    /// The jump status of the object controlled
    /// by the player (true is intention to jump, false is not).
    pub fn jump(&self) -> bool {
        self.jump
    }

    /// The crouch status of the player (true is intention to crouch, false is not)
    pub fn crouch(&self) -> bool {
        self.crouch
    }

    /// The third person aim mode of the player
    pub fn aim(&self) -> bool {
        self.aim
    }

    /// Sets the forward moving magnitude of the object
    /// controlled by the player (back is positive, forward is negative).
    pub fn set_move_forward(&mut self, new_magnitude: f32) {
        self.move_forward = new_magnitude;
    }

    /// Sets the right moving magnitude of the object
    /// controlled by the player (right is positive, left is negative).
    pub fn set_move_right(&mut self, new_magnitude: f32) {
        self.move_right = new_magnitude;
    }

    /// Sets the look-up magnitude of the object
    /// controlled by the player (up is positive, down is negative).
    pub fn set_rotate_up(&mut self, new_magnitude: f32) {
        self.rotate_up = new_magnitude;
    }

    /// Sets the right turn magnitude of the object
    /// controlled by the player (right is positive, left is negative).
    pub fn set_rotate_right(&mut self, new_magnitude: f32) {
        self.rotate_right = new_magnitude;
    }

    /// Sets the jump status of the object controlled
    /// by the player (true is intention to jump, false is not).
    pub fn set_jump(&mut self, jump_state: bool) {
        self.jump = jump_state;
    }

    /// Sets the crouch status of the player (true is intention to crouch, false is not)
    pub fn set_crouch(&mut self, crouch_state: bool) {
        self.crouch = crouch_state;
    }

    /// Sets the aim status of the player (true is intention to aim, false is not)
    pub fn set_aim(&mut self, aim_state: bool) {
        self.aim = aim_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_toml() {
        // Test normal conditions
        let input = Input::try_from_toml(
            "
        move_forward = 1.0
        move_right = 3.0
        rotate_up = 5.0
        rotate_right = 7
        jump = true
        crouch = true
        ",
        )
        .unwrap();

        assert_eq!(input.move_forward(), 1.0);
        assert_eq!(input.move_right(), 3.0);
        assert_eq!(input.rotate_up(), 5.0);
        assert_eq!(input.rotate_right(), 7.0);
        assert_eq!(input.jump(), true);
        assert_eq!(input.crouch(), true);
    }

    #[test]
    fn try_to_toml() {
        // Test normal conditions
        let input = Input::default();
        let input_toml = input.try_to_toml().unwrap();

        assert_eq!(
            input_toml,
            "move_forward = 0.0\nmove_right = 0.0\nrotate_up = 0.0\nrotate_right = 0.0\njump = false\ncrouch = false\n"
        );
    }
}
