//! # unsafe_math proc macro
//!
//! This crate contains the proc macro implementation for `unsafe_math`
//! The macro replaces binary operations with calls to "fast" trait methods

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    BinOp, Expr, Stmt, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    visit_mut::{self, VisitMut},
};

struct UnsafeMathVisitor;

impl VisitMut for UnsafeMathVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        // strip any parentheses around the current expression
        // (otherwise unneсessary parentheses may appear since we introduce function calls which already have parentheses)
        while let Expr::Paren(expr_paren) = expr {
            *expr = *expr_paren.expr.clone();
        }

        // visit children before
        visit_mut::visit_expr_mut(self, expr);

        // replace binary operations with fast ones
        if let Expr::Binary(syn::ExprBinary {
            left, op, right, ..
        }) = expr
            && let Some(method) = binary_op_to_method_name(op)
        {
            let rewritten = quote! { ::unsafe_math_trait::UnsafeMath::#method(#left, #right) };

            *expr = match op {
                // for compound assigns, we assign the result back to the left expression
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
                    syn::parse_quote! { #left = #rewritten }
                }
                // for regular binary ops, we just replace the expression
                _ => {
                    syn::parse_quote! { #rewritten }
                }
            };
        }
    }
}

/// Returns name of corresponding function in UnsafeMath trait, if any.
fn binary_op_to_method_name(op: &BinOp) -> Option<Ident> {
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
    Some(Ident::new(name, Span::call_site()))
}

struct StmtWithComma(Stmt);
impl Parse for StmtWithComma {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let stmt: Stmt = input.parse()?;
        // consume leftover comma if presented
        let _ = input.parse::<Token![,]>();
        Ok(StmtWithComma(stmt))
    }
}

/// Main `unsafe_math` macro. Replaces all binary operations with their unchecked/f_fast versions.
#[proc_macro_attribute]
pub fn unsafe_math(_args: TokenStream, item: TokenStream) -> TokenStream {
    let StmtWithComma(mut stmt) = parse_macro_input!(item as StmtWithComma);
    let mut visitor = UnsafeMathVisitor;
    visitor.visit_stmt_mut(&mut stmt);
    TokenStream::from(quote! { #stmt })
}

/// Version of `unsafe_math` macro that wraps statements. Replaces all binary operations with their unchecked/f_fast versions.
#[proc_macro]
pub fn unsafe_math_block(input: TokenStream) -> TokenStream {
    let mut stmts: Vec<Stmt> = syn::parse_macro_input!(input with syn::Block::parse_within);
    let mut visitor = UnsafeMathVisitor;
    for stmt in &mut stmts {
        visitor.visit_stmt_mut(stmt);
    }
    TokenStream::from(quote!({ #(#stmts)* }))
}
