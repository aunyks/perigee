pub use crate::ffi::interface_commands::*;
pub use crate::ffi::strings::*;

pub mod bytes;
pub mod interface_commands;
pub mod strings;

/// Dereference a pointer to the item it points
/// to in memory and borrow it mutably.
///
/// Safety assumptions:
/// - `ptr` absolutely *cannot* be null, otherwise the program will panic
///    or see undefined behavior.
pub unsafe fn from_mut_ptr<'a, T>(ptr: *mut T) -> &'a mut T {
    &mut *ptr
}

/// Dereference a pointer to the item it points
/// to in memory and borrow it.
///
/// Safety assumptions:
/// - `ptr` absolutely *cannot* be null, otherwise the program will panic
///    or see undefined behavior.
pub unsafe fn from_ptr<'a, T>(ptr: *const T) -> &'a T {
    &*ptr
}
