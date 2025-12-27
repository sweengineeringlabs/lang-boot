use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct RateLimitArgs {
    requests: usize,
    window: u64,
}

pub fn impl_rate_limit(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = RateLimitArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    
    let limiter_name = syn::Ident::new(
        &format!("__LIMITER_{}", func_name_str.to_uppercase()),
        proc_macro2::Span::call_site(),
    );

    let requests = args.requests;
    let window = args.window;

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    // Generate rate limiter initialization
    let limiter_init = quote! {
        static #limiter_name: ::std::sync::OnceLock<
            ::std::sync::Arc<::rustboot_ratelimit::TokenBucket>
        > = ::std::sync::OnceLock::new();
    };

    let wrapped = if is_async {
        quote! {
            #limiter_init

            #visibility #sig {
                let limiter = #limiter_name.get_or_init(|| {
                    ::std::sync::Arc::new(
                        ::rustboot_ratelimit::TokenBucket::new(
                            #requests,
                            ::std::time::Duration::from_secs(#window)
                        )
                    )
                });

                // Try to acquire token
                limiter.try_acquire().await?;

                // Execute original function
                (async #block).await
            }
        }
    } else {
        quote! {
            #limiter_init

            #visibility #sig {
                let limiter = #limiter_name.get_or_init(|| {
                    ::std::sync::Arc::new(
                        ::rustboot_ratelimit::TokenBucket::new(
                            #requests,
                            ::std::time::Duration::from_secs(#window)
                        )
                    )
                });

                // Try to acquire token (blocking)
                ::std::thread::sleep(::std::time::Duration::from_millis(10));
                
                // Execute original function
                (|| #block)()
            }
        }
    };

    Ok(wrapped)
}
