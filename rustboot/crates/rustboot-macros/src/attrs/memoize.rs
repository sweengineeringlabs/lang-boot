use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

pub fn impl_memoize(_args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let func = &input;
    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    
    let cache_name = syn::Ident::new(
        &format!("__MEMO_{}", func_name_str.to_uppercase()),
        proc_macro2::Span::call_site(),
    );

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let _output = &sig.output;

    // Extract parameter names for cache key
    let params = &sig.inputs;
    let param_names: Vec<_> = params.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                return Some(&pat_ident.ident);
            }
        }
        None
    }).collect();

    // Generate cache initialization
    let cache_init = quote! {
        static #cache_name: ::std::sync::OnceLock<
            ::std::sync::Mutex<::std::collections::HashMap<String, _>>
        > = ::std::sync::OnceLock::new();
    };

    // Generate cache key from parameters
    let cache_key = quote! {
        format!("{}(#({}={:?}),*)", #func_name_str, #(stringify!(#param_names), &#param_names),*)
    };

    // Generate wrapped function
    let wrapped = quote! {
        #cache_init

        #visibility #sig {
            let cache = #cache_name.get_or_init(|| {
                ::std::sync::Mutex::new(::std::collections::HashMap::new())
            });

            // Generate cache key
            let key = #cache_key;

            // Try to get from cache
            {
                let cache_guard = cache.lock().unwrap();
                if let Some(cached_value) = cache_guard.get(&key) {
                    return cached_value.clone();
                }
            }

            // Execute original function
            let result = (|| #block)();

            // Store in cache
            {
                let mut cache_guard = cache.lock().unwrap();
                cache_guard.insert(key, result.clone());
            }

            result
        }
    };

    Ok(wrapped)
}
