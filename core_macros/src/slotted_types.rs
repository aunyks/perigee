use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Token, Type,
};

use crate::shared::generate_type_slotting_functions;

struct SlottedTypes {
    types_to_slot: Vec<Type>,
}

impl Parse for SlottedTypes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut types_to_slot = Vec::new();
        while let Ok(slotted_type) = input.parse::<Type>() {
            types_to_slot.push(slotted_type);
            // Grab a comma and throw it out of the way.
            // If no comma exists, then stop looping
            if input.parse::<Token![,]>().is_err() {
                break;
            }
        }
        Ok(Self { types_to_slot })
    }
}

pub fn slotted_types(input: TokenStream) -> TokenStream {
    let SlottedTypes { types_to_slot } = parse_macro_input!(input);

    generate_type_slotting_functions(&types_to_slot)
}
