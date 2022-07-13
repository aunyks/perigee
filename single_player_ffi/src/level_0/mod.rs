use crate::types::*;
use perigee_single_player::core::events::GameEvent;
use perigee_single_player::level_0::Sim;

/// Dereference a pointer to the item it points
/// to in memory.
///
/// Security assumptions:
/// - `ptr` absolutely *cannot* be null, otherwise the program will panic
fn from_mut_ptr<T>(ptr: *mut T) -> &'static mut T {
    unsafe { &mut *ptr }
}

#[cfg(not(test))]
extern "C" {
    fn on_event(event_type: u32, extra_data_1: u32, extra_data_2: u32);
}
#[cfg(test)]
fn on_event(_event_type: u32, _extra_data_1: u32, _extra_data_2: u32) {}

// The below 3 (allocate_*_space) functions are used to allocate
// space on the heap at startup to be constantly reused for returning
// their respective types back to the host runtime. Doing so without using
// the heap would force us to return references to stack variables, which would
// cause undefined behavior. For another explanation, see here:
// https://stackoverflow.com/questions/30826757/stack-behavior-when-returning-a-pointer-to-local-variable
//
// There's no need to manually deallocate these, as they exist throughout the lifetime
// of the simulation and will be freed by the runtime when the entire simulation is no longer in use.

#[no_mangle]
pub extern "C" fn allocate_vector_space() -> *mut CVector {
    Box::into_raw(Box::new(CVector::default()))
}

#[no_mangle]
pub extern "C" fn allocate_quaternion_space() -> *mut CQuaternion {
    Box::into_raw(Box::new(CQuaternion::default()))
}

#[no_mangle]
pub extern "C" fn allocate_isometry_space() -> *mut CIsometry {
    Box::into_raw(Box::new(CIsometry::default()))
}

#[no_mangle]
pub extern "C" fn free_vector_space(vector_space: *mut CVector) {
    unsafe { Box::from_raw(vector_space) };
}

#[no_mangle]
pub extern "C" fn free_quaternion_space(quaternion_space: *mut CQuaternion) {
    unsafe { Box::from_raw(quaternion_space) };
}

#[no_mangle]
pub extern "C" fn free_isometry_space(isometry_space: *mut CIsometry) {
    unsafe { Box::from_raw(isometry_space) };
}

#[no_mangle]
pub extern "C" fn create_sim() -> *mut Sim<'static> {
    Box::into_raw(Box::new(Sim::new()))
}

#[no_mangle]
pub extern "C" fn initialize_sim(game_ptr: *mut Sim<'static>) {
    let game = from_mut_ptr(game_ptr);
    game.initialize().unwrap_or_else(|_| std::process::abort());
}

#[no_mangle]
pub extern "C" fn settings_left_right_look_sensitivity(game_ptr: *mut Sim<'static>) -> u8 {
    let game = from_mut_ptr(game_ptr);
    game.settings.left_right_look_sensitivity()
}

#[no_mangle]
pub extern "C" fn settings_up_down_look_sensitivity(game_ptr: *mut Sim<'static>) -> u8 {
    let game = from_mut_ptr(game_ptr);
    game.settings.up_down_look_sensitivity()
}

#[no_mangle]
pub extern "C" fn settings_set_left_right_look_sensitivity(
    game_ptr: *mut Sim<'static>,
    new_sensitivity: u32,
) {
    let game = from_mut_ptr(game_ptr);
    game.settings
        .set_left_right_look_sensitivity(new_sensitivity as u8);
}

#[no_mangle]
pub extern "C" fn settings_set_up_down_look_sensitivity(
    game_ptr: *mut Sim<'static>,
    new_sensitivity: u32,
) {
    let game = from_mut_ptr(game_ptr);
    game.settings
        .set_up_down_look_sensitivity(new_sensitivity as u8);
}

#[no_mangle]
pub extern "C" fn input_set_move_forward(game_ptr: *mut Sim<'static>, new_magnitude: f32) {
    let game = from_mut_ptr(game_ptr);
    game.input.set_move_forward(new_magnitude);
}

#[no_mangle]
pub extern "C" fn input_set_move_right(game_ptr: *mut Sim<'static>, new_magnitude: f32) {
    let game = from_mut_ptr(game_ptr);
    game.input.set_move_right(new_magnitude);
}

#[no_mangle]
pub extern "C" fn input_set_rotate_up(game_ptr: *mut Sim<'static>, new_magnitude: f32) {
    let game = from_mut_ptr(game_ptr);
    game.input.set_rotate_up(new_magnitude);
}

#[no_mangle]
pub extern "C" fn input_set_rotate_right(game_ptr: *mut Sim<'static>, new_magnitude: f32) {
    let game = from_mut_ptr(game_ptr);
    game.input.set_rotate_right(new_magnitude);
}

#[no_mangle]
pub extern "C" fn input_set_jump(game_ptr: *mut Sim<'static>, jump_val: u8) {
    let game = from_mut_ptr(game_ptr);
    game.input.set_jump(jump_val > 0);
}

#[no_mangle]
pub extern "C" fn input_set_crouch(game_ptr: *mut Sim<'static>, crouch_val: u8) {
    let game = from_mut_ptr(game_ptr);
    game.input.set_crouch(crouch_val > 0);
}

#[no_mangle]
pub extern "C" fn step_sim(game_ptr: *mut Sim<'static>, delta_seconds: f32) {
    let game = from_mut_ptr(game_ptr);
    game.step(delta_seconds);
}

#[no_mangle]
pub extern "C" fn get_game_events(game_ptr: *mut Sim<'static>) {
    let game = from_mut_ptr(game_ptr);
    while game.events.size() > 0 {
        let event = game.events.dequeue();
        let mut extra_data_1 = 0;
        let extra_data_2 = 0;

        match event {
            GameEvent::AudioVisual(av_event) => {
                let av_asset = *av_event.asset();
                extra_data_1 = av_asset.into();
            }
            _ => {}
        };

        // The event type is already encoded the below `.into()` statement
        #[allow(unused_unsafe)]
        unsafe {
            on_event(event.into(), extra_data_1, extra_data_2);
        }
    }
}

#[no_mangle]
pub extern "C" fn head_global_translation(game_ptr: *mut Sim<'static>, vector_space: *mut CVector) {
    let game = from_mut_ptr(game_ptr);
    let head_global_isometry = game.player.body_isometry() * game.player.head_isometry();
    let trans: CVector = head_global_isometry.translation.into();
    unsafe {
        *vector_space = trans;
    };
}

#[no_mangle]
pub extern "C" fn head_global_rotation(game_ptr: *mut Sim<'static>, quat_space: *mut CQuaternion) {
    let game = from_mut_ptr(game_ptr);
    let head_global_isometry = game.player.body_isometry() * game.player.head_isometry();
    let quat: CQuaternion = head_global_isometry.rotation.into();
    unsafe {
        *quat_space = quat;
    };
}

#[no_mangle]
pub extern "C" fn destroy_sim(game_ptr: *mut Sim<'static>) {
    // Box will deallocate the memory on drop
    unsafe { Box::from_raw(game_ptr) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_lifecycle() {
        let game_ptr = create_sim();
        initialize_sim(game_ptr);
        let game = from_mut_ptr(game_ptr);

        let head_global_isometry = game.player.body_isometry() * game.player.head_isometry();
        let a = head_global_isometry.translation;
        println!("{:?}", a);
        for _ in 0..200 {
            step_sim(game_ptr, 1.0 / 60.0);
        }
        let game = from_mut_ptr(game_ptr);
        let head_global_isometry = game.player.body_isometry() * game.player.head_isometry();
        let b = head_global_isometry.translation;
        println!("{:?}", b);
        destroy_sim(game_ptr);
    }
}
