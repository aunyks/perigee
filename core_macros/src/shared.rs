use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Ident, Type};

pub fn generate_type_slotting_functions(types_to_slot: &Vec<Type>) -> TokenStream {
    let mut final_expansion = TokenStream::new();

    for (i, type_to_slot) in types_to_slot.iter().enumerate() {
        let lowercase_type_name = type_to_slot
            .clone()
            .into_token_stream()
            .to_string()
            .to_lowercase()
            .replace(&['[', ']', ';', ' '], "")
            .replace(&['<', '>', ','], "_")
            .replace("()", "empty");
        let alloc_fn_name = Ident::new(
            &format!("allocate_{}_space", lowercase_type_name),
            Span::call_site(),
        );
        let free_fn_name = Ident::new(
            &format!("free_{}_space", lowercase_type_name),
            Span::call_site(),
        );

        let copy_assertion_struct_name =
            Ident::new(&format!("_AssertCopy{}", i), Span::call_site());
        let default_assertion_struct_name =
            Ident::new(&format!("_AssertDefault{}", i), Span::call_site());

        let assert_type_impls_copy = quote_spanned! {type_to_slot.span()=>
            struct #copy_assertion_struct_name where #type_to_slot: std::marker::Copy;
        };

        let assert_type_impls_default = quote_spanned! {type_to_slot.span()=>
            struct #default_assertion_struct_name where #type_to_slot: std::default::Default;
        };

        let type_name_identifier = Ident::new(&lowercase_type_name, Span::call_site());

        let type_expansion = quote! {
            #assert_type_impls_copy
            #assert_type_impls_default

            #[no_mangle]
            pub extern "C" fn #alloc_fn_name() -> *mut #type_to_slot {
                Box::into_raw(Box::new(<#type_to_slot>::default()))
            }

            #[no_mangle]
            pub extern "C" fn #free_fn_name(#type_name_identifier: *mut #type_to_slot) {
                unsafe { Box::from_raw(#type_name_identifier) };
            }
        };

        final_expansion.extend::<TokenStream>(type_expansion.into());
    }

    TokenStream::from(final_expansion)
}
