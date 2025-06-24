//! # unsafe_math proc macro
//!
//! This crate contains the proc macro implementation for `unsafe_math`
//! The macro replaces binary operations with calls to "fast" trait methods

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    BinOp, Expr, ExprBinary, ItemFn, parse_macro_input,
    visit_mut::{self, VisitMut},
};

struct UnsafeMathVisitor;

impl VisitMut for UnsafeMathVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);

        if let Expr::Binary(ExprBinary {
            left, op, right, ..
        }) = expr
            && let Some(method) = binary_op_to_method_name(op)
        {
            // If its compound-assign (+=, -=, ...), emit `left = left.fast_xyz(right)`
            // Else emit just `left.fast_xyz(right)`
            let rewritten = match op {
                BinOp::AddAssign(_)
                | BinOp::SubAssign(_)
                | BinOp::MulAssign(_)
                | BinOp::DivAssign(_)
                | BinOp::RemAssign(_)
                | BinOp::ShlAssign(_)
                | BinOp::ShrAssign(_)
                | BinOp::BitAndAssign(_)
                | BinOp::BitOrAssign(_)
                | BinOp::BitXorAssign(_) => {
                    quote! { #left = (#left).#method(#right) }
                }
                _ => {
                    quote! { (#left).#method(#right) }
                }
            };

            *expr = syn::parse_quote! { #rewritten };
        }
    }
}

fn binary_op_to_method_name(op: &BinOp) -> Option<proc_macro2::Ident> {
    // TODO: is Ident::new(...) free?
    let name = match op {
        BinOp::Add(_) | BinOp::AddAssign(_) => "fast_add",
        BinOp::Sub(_) | BinOp::SubAssign(_) => "fast_sub",
        BinOp::Mul(_) | BinOp::MulAssign(_) => "fast_mul",
        BinOp::Div(_) | BinOp::DivAssign(_) => "fast_div",
        BinOp::Rem(_) | BinOp::RemAssign(_) => "fast_rem",
        BinOp::Shl(_) | BinOp::ShlAssign(_) => "fast_shl",
        BinOp::Shr(_) | BinOp::ShrAssign(_) => "fast_shr",
        _ => return None,
    };
    Some(syn::Ident::new(name, proc_macro2::Span::call_site()))
}

#[proc_macro_attribute]
pub fn unsafe_math(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    let mut visitor = UnsafeMathVisitor;
    visitor.visit_block_mut(&mut function.block);

    TokenStream::from(quote! { #function })
}
