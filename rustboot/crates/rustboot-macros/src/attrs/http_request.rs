//! HTTP request attribute macro for declarative API definitions.
//!
//! This macro simplifies defining HTTP API client methods by generating
//! the boilerplate for making HTTP requests.

use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
pub struct HttpRequestArgs {
    /// HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
    method: String,
    /// URL path, can include placeholders like {id}
    path: String,
    /// Whether the first argument is the request body (for POST, PUT, PATCH)
    #[darling(default)]
    body: bool,
    /// Content type for the request body
    #[darling(default = "default_content_type")]
    content_type: String,
    /// Expected response content type
    #[darling(default = "default_content_type")]
    #[allow(dead_code)]
    response_type: String,
}

fn default_content_type() -> String {
    "application/json".to_string()
}

pub fn impl_http_request(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = HttpRequestArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    let visibility = &func.vis;
    let sig = &func.sig;
    let attrs = &func.attrs;

    // Parse HTTP method - use local Method type (user must import it)
    let http_method = match args.method.to_uppercase().as_str() {
        "GET" => quote! { Method::Get },
        "POST" => quote! { Method::Post },
        "PUT" => quote! { Method::Put },
        "DELETE" => quote! { Method::Delete },
        "PATCH" => quote! { Method::Patch },
        "HEAD" => quote! { Method::Head },
        "OPTIONS" => quote! { Method::Options },
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Unknown HTTP method: {}", args.method),
            ))
        }
    };

    let path = &args.path;
    let content_type = &args.content_type;

    // Get function parameters (skip self)
    let params: Vec<_> = sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(ident) = &*pat_type.pat {
                    return Some((ident.ident.clone(), pat_type.ty.clone()));
                }
            }
            None
        })
        .collect();

    // Build URL with path parameter substitution
    let url_construction = if path.contains('{') {
        // Path has placeholders like /users/{id}
        let mut format_str = path.clone();
        let mut format_args = Vec::new();

        for (param_name, _) in &params {
            let placeholder = format!("{{{}}}", param_name);
            if format_str.contains(&placeholder) {
                format_str = format_str.replace(&placeholder, "{}");
                format_args.push(quote! { #param_name });
            }
        }

        if format_args.is_empty() {
            quote! {
                let __url = format!("{}{}", self.base_url, #path);
            }
        } else {
            quote! {
                let __url = format!(concat!("{}", #format_str), self.base_url, #(#format_args),*);
            }
        }
    } else {
        quote! {
            let __url = format!("{}{}", self.base_url, #path);
        }
    };

    // Build request body if needed
    let body_construction = if args.body && !params.is_empty() {
        // Find the body parameter (typically the last non-path parameter)
        let body_param = params.last().map(|(name, _)| name);
        if let Some(body_name) = body_param {
            if content_type == "application/json" {
                quote! {
                    let __request = __request.json(&#body_name)
                        .map_err(|e| HttpError::Request(format!("Serialization error: {}", e)))?;
                }
            } else {
                quote! {
                    let __body_bytes: Vec<u8> = #body_name.into();
                    let __request = __request.body(__body_bytes);
                }
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    };

    // Determine if we need to set content-type header
    let content_type_header = if args.body {
        quote! {
            let __request = __request.header("Content-Type".to_string(), #content_type.to_string());
        }
    } else {
        quote! {}
    };

    // Generate the wrapper function
    // Note: User must have `use crate::{HttpClient, HttpError, Method, Request}` or equivalent in scope
    let wrapped = quote! {
        #(#attrs)*
        #visibility #sig {
            #url_construction

            let __request = Request::new(#http_method, __url);

            #content_type_header
            #body_construction

            let __response = self.client.send(__request).await?;

            if __response.is_success() {
                __response.json()
                    .map_err(|e| HttpError::Request(format!("Deserialization error: {}", e)))
            } else {
                Err(HttpError::Request(
                    format!("HTTP {} from {}: {}", __response.status, #func_name_str,
                        __response.text().unwrap_or_else(|_| "Unknown error".to_string()))
                ))
            }
        }
    };

    Ok(wrapped)
}
