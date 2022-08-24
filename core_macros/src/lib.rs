use proc_macro::TokenStream;

mod ffi;
mod shared;
mod slotted_types;

#[proc_macro]
pub fn slotted_types(input: TokenStream) -> TokenStream {
    slotted_types::slotted_types(input)
}

#[proc_macro_attribute]
pub fn ffi(args: TokenStream, input: TokenStream) -> TokenStream {
    ffi::ffi(args, input)
}

/// This is a marker attribute used by #[ffi]. It's a no-op.
#[proc_macro_attribute]
pub fn slot_return(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// This is a marker attribute used by #[ffi]. It's a no-op.
#[proc_macro_attribute]
pub fn ffi_only(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// This is a marker attribute used by #[ffi]. It's a no-op.
#[proc_macro_attribute]
pub fn ffi_never(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
