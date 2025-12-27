#![allow(dead_code)]
/// Utility functions for macro implementations (Syn 2.0 compatible)
use syn::{Attribute, Error, Expr, Lit, Meta, Result};

/// Extract string literal from attribute meta
pub fn get_string_from_meta(meta: &Meta) -> Result<Option<String>> {
    if let Meta::NameValue(nv) = meta {
        if let Expr::Lit(expr_lit) = &nv.value {
            if let Lit::Str(lit) = &expr_lit.lit {
                return Ok(Some(lit.value()));
            }
        }
    }
    Ok(None)
}

/// Extract integer literal from attribute meta
pub fn get_int_from_meta(meta: &Meta) -> Result<Option<i64>> {
    if let Meta::NameValue(nv) = meta {
        if let Expr::Lit(expr_lit) = &nv.value {
            if let Lit::Int(lit) = &expr_lit.lit {
                return Ok(Some(lit.base10_parse()?));
            }
        }
    }
    Ok(None)
}

/// Extract boolean literal from attribute meta
pub fn get_bool_from_meta(meta: &Meta) -> Result<Option<bool>> {
    if let Meta::NameValue(nv) = meta {
        if let Expr::Lit(expr_lit) = &nv.value {
            if let Lit::Bool(lit) = &expr_lit.lit {
                return Ok(Some(lit.value));
            }
        }
    }
    Ok(None)
}

/// Check if meta is a specific path
pub fn is_meta_path(meta: &Meta, name: &str) -> bool {
    match meta {
        Meta::Path(path) => path.is_ident(name),
        _ => false,
    }
}

/// Generate error for unsupported attribute
pub fn unsupported_attr(span: proc_macro2::Span, attr: &str) -> Error {
    Error::new(span, format!("Unsupported attribute: {}", attr))
}

/// Parse nested meta from attribute
pub fn parse_nested_meta<F>(attr: &Attribute, mut f: F) -> Result<()>
where
    F: FnMut(&Meta) -> Result<()>,
{
    match &attr.meta {
        Meta::List(list) => {
            for nested in list.parse_args_with(
                syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
            )? {
                f(&nested)?;
            }
            Ok(())
        }
        meta => f(meta),
    }
}
