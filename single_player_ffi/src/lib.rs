mod level_0;
mod types;

// Level functions are hidden behind
// compile feature flags so that we
// only build one set of FFI functions
// into the library at a time,
// so 1 lib = 1 level

#[cfg(feature = "level_9")]
pub use level_0::*;
