use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct TracedArgs {
    #[darling(default = "default_level")]
    level: String,
    #[darling(default)]
    skip: Option<Vec<syn::LitStr>>,
    #[darling(default)]
    name: Option<String>,
}

fn default_level() -> String {
    "INFO".to_string()
}

pub fn impl_traced(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = TracedArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let func_name = &func.sig.ident;
    let name_storage;
    let func_name_str = match &args.name {
        Some(name) => name,
        None => {
            name_storage = func_name.to_string();
            &name_storage
        }
    };

    // Convert level string to uppercase for tracing::Level
    let level_upper = args.level.to_uppercase();
    let level_ident = syn::Ident::new(&level_upper, proc_macro2::Span::call_site());

    let visibility = &func.vis;
    let sig = &func.sig;
    let attrs = &func.attrs;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    // Generate parameter field expressions for tracing span! macro
    // tracing span! syntax: span!(Level::INFO, "name", field = %value, field2 = ?value2)
    let params = &sig.inputs;
    let param_fields: Vec<_> = params
        .iter()
        .filter_map(|param| {
            if let syn::FnArg::Typed(pat_type) = param {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = &pat_ident.ident;
                    let param_name_str = param_name.to_string();

                    // Skip self and explicitly skipped params
                    if param_name_str == "self"
                        || args
                            .skip
                            .as_ref()
                            .map(|s| s.iter().any(|lit| lit.value() == param_name_str))
                            .unwrap_or(false)
                    {
                        return None;
                    }

                    // Use ?field syntax for Debug formatting
                    return Some(quote! { #param_name = ?#param_name });
                }
            }
            None
        })
        .collect();

    // Build the span! invocation with or without fields
    let span_create = if param_fields.is_empty() {
        quote! {
            ::rustboot_observability::tracing::span!(
                ::rustboot_observability::tracing::Level::#level_ident,
                #func_name_str
            )
        }
    } else {
        quote! {
            ::rustboot_observability::tracing::span!(
                ::rustboot_observability::tracing::Level::#level_ident,
                #func_name_str,
                #(#param_fields),*
            )
        }
    };

    let wrapped = if is_async {
        quote! {
            #(#attrs)*
            #visibility #sig {
                let __traced_span = #span_create;
                let __traced_enter = __traced_span.enter();

                let __traced_start = ::std::time::Instant::now();
                let __traced_result = (async #block).await;
                let __traced_duration = __traced_start.elapsed();

                ::rustboot_observability::tracing::event!(
                    ::rustboot_observability::tracing::Level::#level_ident,
                    duration_ms = __traced_duration.as_millis() as u64,
                    "completed"
                );

                __traced_result
            }
        }
    } else {
        quote! {
            #(#attrs)*
            #visibility #sig {
                let __traced_span = #span_create;
                let __traced_enter = __traced_span.enter();

                let __traced_start = ::std::time::Instant::now();
                let __traced_result = (|| #block)();
                let __traced_duration = __traced_start.elapsed();

                ::rustboot_observability::tracing::event!(
                    ::rustboot_observability::tracing::Level::#level_ident,
                    duration_ms = __traced_duration.as_millis() as u64,
                    "completed"
                );

                __traced_result
            }
        }
    };

    Ok(wrapped)
}
