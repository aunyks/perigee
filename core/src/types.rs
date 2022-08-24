#![allow(non_camel_case_types)]

#[cfg(target_pointer_width = "32")]
pub type fsize = f32;
#[cfg(target_pointer_width = "64")]
pub type fsize = f64;
#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unsupported target pointer width. Expected a 32 or 64 bit target");
