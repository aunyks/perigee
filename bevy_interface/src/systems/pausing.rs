use crate::states::GameLevel;
use bevy::prelude::*;

pub fn pause_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_buttons: Res<Input<GamepadButton>>,
    mut game_level: ResMut<State<GameLevel>>,
) {
    let mut should_pause = false;
    if keyboard_input.just_pressed(KeyCode::Escape) {
        should_pause = true;
        keyboard_input.reset(KeyCode::Escape);
    }
    for gamepad in gamepads.iter().cloned() {
        if gamepad_buttons.just_pressed(GamepadButton(gamepad, GamepadButtonType::Start)) {
            should_pause = true;
        }
    }
    if should_pause {
        game_level
            .push(GameLevel::PauseMenu)
            .expect("Error occurred while trying to pause the game!");
    }
}

pub fn resume_game() {}
