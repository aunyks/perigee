#[cfg(not(feature = "ffi"))]
use log::debug;
#[cfg(feature = "ffi")]
use std::ffi::{c_char, CString};

#[cfg(feature = "ffi")]
extern "C" {
    fn play_2d_audio_hook(audio_name_ptr: *const c_char);
    fn stop_2d_audio_hook(audio_name_ptr: *const c_char);
    fn loop_2d_audio_hook(audio_name_ptr: *const c_char);
    fn loop_animation_hook(scene_obj_name_ptr: *const c_char, anim_name_ptr: *const c_char);
    fn stop_animation_hook(scene_obj_name_ptr: *const c_char, anim_name_ptr: *const c_char);
    fn assistive_device_announce_hook(announcement_msg_ptr: *const c_char);
}

#[cfg(feature = "ffi")]
pub fn play_2d_audio(audio_name: &str) {
    let msg_cstring = CString::new(audio_name)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    unsafe {
        play_2d_audio_hook(msg_cstring.as_ptr());
    }
}

#[cfg(not(feature = "ffi"))]
pub fn play_2d_audio(audio_name: &str) {
    debug!("Play 2D Audio: {}", audio_name);
}

#[cfg(feature = "ffi")]
pub fn stop_2d_audio(audio_name: &str) {
    let msg_cstring = CString::new(audio_name)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    unsafe {
        stop_2d_audio_hook(msg_cstring.as_ptr());
    }
}

#[cfg(not(feature = "ffi"))]
pub fn stop_2d_audio(audio_name: &str) {
    debug!("Stop 2D Audio: {}", audio_name);
}

#[cfg(feature = "ffi")]
pub fn loop_2d_audio(audio_name: &str) {
    let msg_cstring = CString::new(audio_name)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    unsafe {
        loop_2d_audio_hook(msg_cstring.as_ptr());
    }
}

#[cfg(not(feature = "ffi"))]
pub fn loop_2d_audio(audio_name: &str) {
    debug!("Loop 2D Audio: {}", audio_name);
}

#[cfg(feature = "ffi")]
pub fn loop_animation(scene_object_name: &str, anim_name: &str) {
    let obj_cstring = CString::new(scene_object_name)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    let anim_cstring = CString::new(anim_name)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    unsafe {
        loop_animation_hook(obj_cstring.as_ptr(), anim_cstring.as_ptr());
    }
}

#[cfg(not(feature = "ffi"))]
pub fn loop_animation(scene_object_name: &str, anim_name: &str) {
    debug!(
        "Play Animation: (Scene Object: {}, Animation Name: {})",
        scene_object_name, anim_name,
    );
}

#[cfg(feature = "ffi")]
pub fn stop_animation(scene_object_name: &str, anim_name: &str) {
    let obj_cstring = CString::new(scene_object_name)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    let anim_cstring = CString::new(anim_name)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    unsafe {
        stop_animation_hook(obj_cstring.as_ptr(), anim_cstring.as_ptr());
    }
}

#[cfg(not(feature = "ffi"))]
pub fn stop_animation(scene_object_name: &str, anim_name: &str) {
    debug!(
        "Stop Animation: (Scene Object: {}, Animation Name: {})",
        scene_object_name, anim_name,
    );
}

#[cfg(feature = "ffi")]
pub fn assistive_device_announce(announcement_msg: &str) {
    let announcement_cstring = CString::new(announcement_msg)
        .unwrap_or(CString::new("Unknown string received. Something's wrong").unwrap());
    unsafe { assistive_device_announce_hook(announcement_cstring.as_ptr()) }
}

#[cfg(not(feature = "ffi"))]
pub fn assistive_device_announce(announcement_msg: &str) {
    debug!("Assistive Device Announcement: {}", announcement_msg);
}
