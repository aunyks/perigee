#[no_mangle]
pub extern "C" fn alloc_bytes(num_bytes: usize) -> *mut u8 {
    let reserved_bytes: Vec<u8> = vec![7; num_bytes];
    let mut boxed_byte_slice = reserved_bytes.into_boxed_slice();

    let slice_ptr = boxed_byte_slice.as_mut_ptr();
    std::mem::forget(boxed_byte_slice);

    slice_ptr
}

#[no_mangle]
pub extern "C" fn free_bytes(bytes_ptr: *mut u8, slice_len: usize) {
    unsafe { Vec::from_raw_parts(bytes_ptr, slice_len, slice_len) };
}
