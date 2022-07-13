use crate::states::FirstPersonControlSettings;
use bevy::prelude::*;

/// TL;DR: This plugin enables first person controls for an entity. It's configured using [`FirstPersonControlSettings`](crate::states::FirstPersonControlSettings).
///
/// This plugin adds the [`first_person_movement`](crate::systems::first_person_movement) and
/// [`first_person_lookaround`](crate::systems::first_person_lookaround) systems. The systems will only
/// execute if a [`FirstPersonControlSettings`](crate::states::FirstPersonControlSettings) state has been
/// added to the App and is set to `Enabled`.
///
/// Note: An entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component must exist when this plugin is
/// enabled for it to function, otherwise a panic may occur.
pub struct FirstPersonControlPlugin;

impl Plugin for FirstPersonControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(FirstPersonControlSettings::Enabled).with_system(lock_pointer),
        )
        // .add_system_set(
        //     SystemSet::on_update(FirstPersonControlSettings::Enabled)
        //         .with_system(first_person_movement)
        //         .with_system(first_person_lookaround),
        // )
        .add_system_set(
            SystemSet::on_exit(FirstPersonControlSettings::Enabled).with_system(unlock_pointer),
        );
    }
}

fn lock_pointer(mut windows: ResMut<Windows>) {
    debug!("Locking cursor");
    let window = windows.get_primary_mut().expect(
        "Expected to find window while locking pointer for FirstPersonControlPlugin. None found!",
    );
    window.set_cursor_position(Vec2::new(window.width() / 2f32, window.height() / 2f32));
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);
}

fn unlock_pointer(mut windows: ResMut<Windows>) {
    debug!("Unlocking cursor");
    let window = windows.get_primary_mut().expect(
        "Expected to find window while unlocking pointer for FirstPersonControlPlugin. None found!",
    );
    window.set_cursor_visibility(true);
    window.set_cursor_lock_mode(false);
}
