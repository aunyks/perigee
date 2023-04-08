//! Functions that communicate directly with the
//! interface when compiled to WebAssembly.
#[cfg(not(feature = "ffi"))]
use log::debug;

#[cfg(feature = "ffi")]
extern "C" {
    fn play_audio_hook(
        scene_obj_name_ptr: *const u8,
        scene_obj_name_len: usize,
        audio_name_ptr: *const u8,
        audio_name_len: usize,
        playback_rate: f32,
        volume: f32,
    );
    fn loop_audio_hook(
        scene_obj_name_ptr: *const u8,
        scene_obj_name_len: usize,
        audio_name_ptr: *const u8,
        audio_name_len: usize,
        playback_rate: f32,
        volume: f32,
    );
    fn stop_audio_hook(
        scene_obj_name_ptr: *const u8,
        scene_obj_name_len: usize,
        audio_name_ptr: *const u8,
        audio_name_len: usize,
    );
    fn play_animation_hook(
        scene_obj_name_ptr: *const u8,
        scene_obj_name_len: usize,
        anim_name_ptr: *const u8,
        anim_name_len: usize,
        time_scale: f32,
    );
    fn loop_animation_hook(
        scene_obj_name_ptr: *const u8,
        scene_obj_name_len: usize,
        anim_name_ptr: *const u8,
        anim_name_len: usize,
        time_scale: f32,
    );
    fn stop_animation_hook(
        scene_obj_name_ptr: *const u8,
        scene_obj_name_len: usize,
        anim_name_ptr: *const u8,
        anim_name_len: usize,
    );
    fn assistive_device_announce_hook(
        announcement_msg_name_ptr: *const u8,
        announcement_msg_name_len: usize,
    );
}
#[cfg(feature = "ffi")]
pub fn play_audio(scene_object_name: &str, audio_name: &str, playback_rate: f32, volume: f32) {
    unsafe {
        play_audio_hook(
            scene_object_name.as_ptr(),
            scene_object_name.len(),
            audio_name.as_ptr(),
            audio_name.len(),
            playback_rate,
            volume,
        );
    }
}

/// Play the named audio track once from the perspective of the active camera.
#[cfg(not(feature = "ffi"))]
pub fn play_audio(scene_object_name: &str, audio_name: &str, playback_rate: f32, volume: f32) {
    debug!(
        "Play Audio: (Scene Object: {}, Audio Name: {}, Playback Rate: {}, Volume: {})",
        scene_object_name, audio_name, playback_rate, volume
    );
}

#[cfg(feature = "ffi")]
pub fn loop_audio(scene_object_name: &str, audio_name: &str, playback_rate: f32, volume: f32) {
    unsafe {
        loop_audio_hook(
            scene_object_name.as_ptr(),
            scene_object_name.len(),
            audio_name.as_ptr(),
            audio_name.len(),
            playback_rate,
            volume,
        );
    }
}

/// Repeatedly play the named audio track from the perspective of the active camera until
/// told to stop.
#[cfg(not(feature = "ffi"))]
pub fn loop_audio(scene_object_name: &str, audio_name: &str, playback_rate: f32, volume: f32) {
    debug!(
        "Loop Audio: (Scene Object: {}, Audio Name: {}, Playback Rate: {}, Volume: {})",
        scene_object_name, audio_name, playback_rate, volume
    );
}

#[cfg(feature = "ffi")]
pub fn stop_audio(scene_object_name: &str, audio_name: &str) {
    unsafe {
        stop_audio_hook(
            scene_object_name.as_ptr(),
            scene_object_name.len(),
            audio_name.as_ptr(),
            audio_name.len(),
        );
    }
}

/// Stop playing the named audio track from the perspective of the active camera.
#[cfg(not(feature = "ffi"))]
pub fn stop_audio(scene_object_name: &str, audio_name: &str) {
    debug!(
        "Stop Audio: (Scene Object: {}, Audio Name: {})",
        scene_object_name, audio_name
    );
}

#[cfg(feature = "ffi")]
pub fn play_animation(scene_object_name: &str, anim_name: &str, time_scale: f32) {
    unsafe {
        play_animation_hook(
            scene_object_name.as_ptr(),
            scene_object_name.len(),
            anim_name.as_ptr(),
            anim_name.len(),
            time_scale,
        );
    }
}

/// Repeatedly play the named animation on the named scene object.
#[cfg(not(feature = "ffi"))]
pub fn play_animation(scene_object_name: &str, anim_name: &str, time_scale: f32) {
    debug!(
        "Play Animation: (Scene Object: {}, Animation Name: {}, Time Scale: {})",
        scene_object_name, anim_name, time_scale
    );
}

#[cfg(feature = "ffi")]
pub fn loop_animation(scene_object_name: &str, anim_name: &str, time_scale: f32) {
    unsafe {
        loop_animation_hook(
            scene_object_name.as_ptr(),
            scene_object_name.len(),
            anim_name.as_ptr(),
            anim_name.len(),
            time_scale,
        );
    }
}

/// Repeatedly play the named animation on the named scene object.
#[cfg(not(feature = "ffi"))]
pub fn loop_animation(scene_object_name: &str, anim_name: &str, time_scale: f32) {
    debug!(
        "Loop Animation: (Scene Object: {}, Animation Name: {}, Time Scale: {})",
        scene_object_name, anim_name, time_scale
    );
}

#[cfg(feature = "ffi")]
pub fn stop_animation(scene_object_name: &str, anim_name: &str) {
    unsafe {
        stop_animation_hook(
            scene_object_name.as_ptr(),
            scene_object_name.len(),
            anim_name.as_ptr(),
            anim_name.len(),
        );
    }
}

/// Stop playing the named animation on the named scene object.
#[cfg(not(feature = "ffi"))]
pub fn stop_animation(scene_object_name: &str, anim_name: &str) {
    debug!(
        "Stop Animation: (Scene Object: {}, Animation Name: {})",
        scene_object_name, anim_name,
    );
}

#[cfg(feature = "ffi")]
pub fn assistive_device_announce(announcement_msg_name: &str) {
    unsafe {
        assistive_device_announce_hook(announcement_msg_name.as_ptr(), announcement_msg_name.len());
    }
}

/// Announce the provided named message to the user through an assistive device.
#[cfg(not(feature = "ffi"))]
pub fn assistive_device_announce(announcement_msg_name: &str) {
    debug!(
        "Assistive Device Announcement ID: {}",
        announcement_msg_name
    );
}
