use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    token::{Colon, Comma, Brace},
    Block,
    ReturnType,
    Stmt,
    PatType,
    Visibility,
    FnArg,
    Expr,
    parse_macro_input,
    punctuated::Punctuated,
    AttrStyle, ImplItem, Pat, Type,
    parse_quote, PathArguments
};
use crate::shared::generate_type_slotting_functions;
use crate::ffi::helpers::{attribute_name, replace_self_with_sim};

mod helpers;

struct FfiImplBlock {
    item: syn::ItemImpl,
}

impl Parse for FfiImplBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub fn ffi(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ffi_impl = parse_macro_input!(input as FfiImplBlock);

    let mut expanded_impl_internals = TokenStream::new();
    let mut ffi_code = TokenStream::new();

    let mut bare_sim_type = *ffi_impl.item.self_ty.clone();
    if let Type::Path(bare_sim_type_path) = &mut bare_sim_type {
        for path_seg in bare_sim_type_path.path.segments.iter_mut() {
            path_seg.arguments = PathArguments::None;
        }
    }

    let mut types_to_slot: Vec<Type> = Vec::new();

    for impl_item in ffi_impl.item.items.iter_mut() {
        if let ImplItem::Method(ref mut fn_details) = impl_item {
            // Only use process public functions
            if !matches!(fn_details.vis, Visibility::Public(_)) {
                continue;
            }

            let mut slot_return = false;
            for attr in &mut fn_details.attrs {
                if matches!(attr.style, AttrStyle::Outer) {
                    let attribute_name = attribute_name(&attr);
                    if attribute_name == "slot_return" {
                        slot_return = true;
                    }
                }
            }

            // include the original function if we don't *only* want ffi code
            let original_fn_details = fn_details.clone();
            expanded_impl_internals
                .extend::<TokenStream>(original_fn_details.into_token_stream().into());

            let mut ffi_fn_statements: Vec<Stmt> = Vec::new();
            let mut invocation_args = Vec::new();
            let updated_fn_args_iter = fn_details.sig.inputs.iter().map(|arg| {
                    match &arg {
                        // &self -> sim: *const Sim, &mut self -> sim: *mut Sim
                        FnArg::Receiver(receiver_details) => {
                            let is_ref = receiver_details.reference.is_some();
                            let is_mut = receiver_details.mutability.is_some();
                            if !is_ref {
                                // arg.span().unwrap().error("Found non-ref receiver argument. This may cause undefined behavior in FFI contexts.").emit();
                            }

                            let new_self_arg = if is_mut {
                                let sim_deref_stmt: Stmt = parse_quote!(
                                    let sim = unsafe { from_mut_ptr(sim_ptr) };
                                );
                                ffi_fn_statements.push(sim_deref_stmt);
                                PatType { attrs: Vec::new(), pat: Box::new(Pat::Verbatim(quote!(sim_ptr))), colon_token: Colon::default(), ty: Box::new(Type::Verbatim(quote!(*mut #bare_sim_type)
                                ))}
                            } else {
                                let sim_deref_stmt: Stmt = parse_quote!(
                                    let sim = unsafe { from_ptr(sim_ptr) };
                                );
                                ffi_fn_statements.push(sim_deref_stmt);
                                PatType { attrs: Vec::new(), pat: Box::new(Pat::Verbatim(quote!(sim_ptr))), colon_token: Colon::default(), ty: Box::new(Type::Verbatim(quote!(*const #bare_sim_type)
                                ))}
                            };
                        
                            FnArg::Typed(new_self_arg)
                        }
                        // &T -> *const T, &mut T -> *mut T, &str -> *const CString, T otherwise (Warning: don't use anything other than primitive types as fn args)
                        FnArg::Typed(typed_arg) => {
                            let mut modified_pattype = typed_arg.clone();
                            let original_type = modified_pattype.ty.clone();

                            let new_type = match *original_type {
                                Type::Reference(rfrnce) => {
                                    if let Type::Verbatim(type_tokens) = *rfrnce.elem {  
                                        let var_name = *modified_pattype.pat.clone();
                                        Type::Verbatim(if rfrnce.mutability.is_some() {
                                            let type_deref_statement: Stmt = parse_quote!(
                                                let #var_name = from_mut_ptr(#var_name);
                                            );
                                            ffi_fn_statements.push(type_deref_statement);
                                            quote!(*mut #type_tokens)
                                        } else {
                                            let type_deref_statement: Stmt = parse_quote!(
                                                let #var_name = from_ptr(#var_name);
                                            );
                                            ffi_fn_statements.push(type_deref_statement);
                                            quote!(*const #type_tokens)
                                        })
                                    } else if let Type::Path(type_path) = *rfrnce.elem {
                                        if let Some(tpath) = type_path.path.get_ident() {
                                            if tpath == "str" {
                                                let var_name = *modified_pattype.pat.clone();
                                                let mut cstring_deref_statements: Vec<Stmt> = parse_quote!(
                                                    let #var_name = unsafe { from_cstring_ptr(#var_name) };
                                                    let #var_name = #var_name
                                                        .to_str()
                                                        .expect("Could not convert CStr to &str. Likely not valid UTF-8");
                                                );
                                                ffi_fn_statements.append(&mut cstring_deref_statements);
                                                Type::Verbatim(quote!(*const std::ffi::CString))
                                            } else {
                                                Type::Path(type_path)
                                            }
                                        } else {
                                            Type::Path(type_path)
                                        }
                                    } else {
                                        // typed_arg.span().unwrap().error("Found reference to complex type. Please just use &T instead of &[T] or whatever you used.").emit();
                                        Type::Reference(rfrnce)
                                    }
                                }
                                Type::Verbatim(tokens) => {
                                    Type::Verbatim(tokens)
                                }
                                _ => {
                                    // typed_arg.span().unwrap().warning("Unsure how to prepare this type for FFI. Leaving it as-is.").emit();
                                    *original_type
                                }
                            };
                            modified_pattype.ty = Box::new(new_type);

                            let arg_pat: Pat = *modified_pattype.pat.clone();
                            let expr: Expr = parse_quote!(#arg_pat);
                            invocation_args.push(expr);

                            FnArg::Typed(modified_pattype)
                        },
                    }
            });

            let mut updated_fn_args: Punctuated<FnArg, Comma> = Punctuated::from_iter(updated_fn_args_iter);

            // add invocation and maybe append a let statement and some slot insertion code
            let invocation_args: Punctuated<Expr, Comma> = Punctuated::from_iter(invocation_args.into_iter());
            let has_return = !matches!(fn_details.sig.output, ReturnType::Default);
            let fn_name = fn_details.sig.ident.clone();
            let mut must_deref_return = false;
            let mut output_fn_return_type: Option<ReturnType> = None;
            if has_return {
                if slot_return {
                    if let ReturnType::Type(_, boxed_type) = &fn_details.sig.output {
                        let return_type = *boxed_type.clone();
                        let return_type: Type = match return_type {
                            Type::Verbatim(_) | Type::Path(_) => return_type,
                            Type::Reference(rfrnce) => {
                                must_deref_return = true;
                                *rfrnce.elem.clone()
                            },
                            _ => {
                                // fn_details.sig.output.span().unwrap().error(format!("Only types (T) and references to types (&T) can be returned from FFI functions. Found {:?}", return_type.clone())).emit();
                                Type::Verbatim(quote!())
                            }
                        };
                        let new_return_type = return_type.clone();
                        if !types_to_slot.contains(&new_return_type) {
                            types_to_slot.push(new_return_type);
                        }
                        ffi_fn_statements.push(parse_quote!(
                            let slotted_value = sim.#fn_name(#invocation_args);
                        ));

                        ffi_fn_statements.push(if must_deref_return {
                            parse_quote!(
                                unsafe {
                                    *return_slot = *slotted_value;
                                };
                            )
                        } else {
                            parse_quote!(
                                unsafe {
                                    *return_slot = slotted_value;
                                };
                            )
                        });
                        updated_fn_args.push(parse_quote!(return_slot: *mut #return_type));
                    } else {
                        // fn_details.sig.output.span().unwrap().error("#[slot_return] used on function with no return type").emit();
                    }
                } else {
                    // leave the return alone if we don't slot it
                    output_fn_return_type = Some(fn_details.sig.output.clone());
                    let mut original_statements = fn_details.block.stmts.clone();
                    replace_self_with_sim(&mut original_statements);
                    ffi_fn_statements.append(&mut original_statements);
                }
            } else {
                let mut original_statements = fn_details.block.stmts.clone();
                replace_self_with_sim(&mut original_statements);
                ffi_fn_statements.append(&mut original_statements);
            }

            // Create block and add to ffi function
            let fn_body_block = Block {
                brace_token: Brace::default(),
                stmts: ffi_fn_statements,
            };
            let ffi_fn_definition = quote!(
                #[no_mangle]
                pub extern "C" fn #fn_name(#updated_fn_args) #output_fn_return_type
                #fn_body_block
            );
            ffi_code.extend::<TokenStream>(
                ffi_fn_definition.into()
            );
        }
    }

    let type_slotting_code = generate_type_slotting_functions(&types_to_slot);
    ffi_code.extend(type_slotting_code);

    let mut expanded_impl_internals: TokenStream2 = expanded_impl_internals.into();
    let ffi_code: TokenStream2 = ffi_code.into();
    // dbg!(quote!(#expanded_impl_internals).to_string());
    quote! {
        impl<'a> #bare_sim_type <'a> {
            #expanded_impl_internals
        }

        unsafe fn from_mut_ptr<'a, T>(ptr: *mut T) -> &'a mut T {
            &mut *ptr
        }

        unsafe fn from_ptr<'a, T>(ptr: *const T) -> &'a T {
            &*ptr
        }

        unsafe fn from_cstring_ptr<'a>(cstr_ptr: *const std::ffi::CString) -> &'a std::ffi::CStr {
            std::ffi::CStr::from_ptr(cstr_ptr as *mut i8)
        }

        #ffi_code
    }.into()
}
