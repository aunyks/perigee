use proc_macro2::Span;
use quote::ToTokens;
use syn::{Attribute, Expr, Ident, PathSegment, Stmt};

pub fn attribute_name(attr: &Attribute) -> Ident {
    attr.path
        .segments
        .last()
        .unwrap_or(&PathSegment {
            ident: Ident::new("__", Span::call_site()),
            arguments: syn::PathArguments::None,
        })
        .ident
        .clone()
}

pub fn replace_self_with_sim(statements: &mut Vec<Stmt>) {
    for stmt in statements {
        let stmt_tokens = stmt.to_token_stream().to_string().replace("self", "sim");
        let new_stmt: Stmt = match syn::parse_str(&stmt_tokens) {
            Ok(a) => a,
            Err(e) => {
                if &e.to_string() == "unexpected end of input, expected semicolon" {
                    let expr: Expr = syn::parse_str(&stmt_tokens).unwrap();
                    Stmt::Expr(expr)
                } else {
                    println!("{}", e.to_string());
                    panic!();
                }
            }
        };
        stmt.clone_from(&new_stmt);
    }
}
