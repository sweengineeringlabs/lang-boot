use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct CircuitBreakerArgs {
    #[darling(default = "default_failure_threshold")]
    failure_threshold: usize,
    #[darling(default = "default_timeout")]
    timeout: u64,
    #[darling(default = "default_success_threshold")]
    success_threshold: usize,
}

fn default_failure_threshold() -> usize {
    5
}

fn default_timeout() -> u64 {
    60
}

fn default_success_threshold() -> usize {
    2
}

pub fn impl_circuit_breaker(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = CircuitBreakerArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    let failure_threshold = args.failure_threshold;
    let timeout = args.timeout;
    let success_threshold = args.success_threshold;

    let wrapped = quote! {
        #visibility #sig {
            use ::rustboot_resilience::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};

            static BREAKER: ::std::sync::OnceLock<::std::sync::Arc<CircuitBreaker>> =
                ::std::sync::OnceLock::new();

            let breaker = BREAKER.get_or_init(|| {
                let config = CircuitBreakerConfig {
                    failure_threshold: #failure_threshold,
                    timeout: ::std::time::Duration::from_secs(#timeout),
                    success_threshold: #success_threshold,
                };
                ::std::sync::Arc::new(CircuitBreaker::new(config))
            });

            breaker.execute(|| async #block).await
        }
    };

    Ok(wrapped)
}
