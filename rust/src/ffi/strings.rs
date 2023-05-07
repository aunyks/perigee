use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn alloc_string(num_chars: usize) -> *mut CString {
    // Init the space with 7s. They'll get updated with the
    // actual string later
    let reserved_bytes: Vec<u8> = vec![7; num_chars];
    unsafe { CString::from_vec_unchecked(reserved_bytes).into_raw() as *mut CString }
}

pub unsafe fn from_cstring_ptr<'a>(cstr_ptr: *const CString) -> &'a CStr {
    CStr::from_ptr(cstr_ptr as *mut i8)
}

#[no_mangle]
pub extern "C" fn free_string(string_ptr: *mut CString) {
    unsafe { CString::from_raw(string_ptr as *mut i8) };
}
