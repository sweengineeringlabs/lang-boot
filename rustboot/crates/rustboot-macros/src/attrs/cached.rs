use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct CachedArgs {
    #[darling(default)]
    ttl: Option<u64>,
    #[darling(default)]
    _capacity: Option<usize>,
}

pub fn impl_cached(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = CachedArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    let cache_name = syn::Ident::new(
        &format!("__CACHE_{}", func_name_str.to_uppercase()),
        proc_macro2::Span::call_site(),
    );

    let ttl = args.ttl.unwrap_or(300);

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    // Extract parameter names for cache key
    let params = &sig.inputs;
    let param_names: Vec<_> = params
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(&pat_ident.ident);
                }
            }
            None
        })
        .collect();

    // Generate cache key from parameters
    let cache_key_expr = if param_names.is_empty() {
        quote! { #func_name_str.to_string() }
    } else {
        quote! { format!("{}({:?})", #func_name_str, (#(&#param_names),*)) }
    };

    let wrapped = if is_async {
        quote! {
            #visibility #sig {
                use ::rustboot_cache::{Cache, InMemoryCache};

                static #cache_name: ::std::sync::OnceLock<InMemoryCache<String, _>> =
                    ::std::sync::OnceLock::new();

                let cache = #cache_name.get_or_init(|| InMemoryCache::new());

                let cache_key = #cache_key_expr;

                // Try to get from cache
                if let Ok(Some(cached_value)) = cache.get(&cache_key) {
                    return cached_value;
                }

                // Execute original function
                let result = (async #block).await;

                // Store in cache with TTL
                let _ = cache.set_with_ttl(
                    cache_key,
                    result.clone(),
                    ::std::time::Duration::from_secs(#ttl)
                );

                result
            }
        }
    } else {
        quote! {
            #visibility #sig {
                use ::rustboot_cache::{Cache, InMemoryCache};

                static #cache_name: ::std::sync::OnceLock<InMemoryCache<String, _>> =
                    ::std::sync::OnceLock::new();

                let cache = #cache_name.get_or_init(|| InMemoryCache::new());

                let cache_key = #cache_key_expr;

                // Try to get from cache
                if let Ok(Some(cached_value)) = cache.get(&cache_key) {
                    return cached_value;
                }

                // Execute original function
                let result = (|| #block)();

                // Store in cache with TTL
                let _ = cache.set_with_ttl(
                    cache_key,
                    result.clone(),
                    ::std::time::Duration::from_secs(#ttl)
                );

                result
            }
        }
    };

    Ok(wrapped)
}
