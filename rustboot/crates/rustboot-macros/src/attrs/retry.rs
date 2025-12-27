use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct RetryArgs {
    #[darling(default = "default_max_attempts")]
    max_attempts: usize,
    /// Name of a function parameter to use for max_attempts (runtime value).
    /// When set, this overrides max_attempts with the runtime parameter value.
    #[darling(default)]
    max_attempts_param: Option<String>,
    /// Name of a function parameter of type RetryConfig to use for all retry settings.
    /// When set, reads max_attempts, initial_delay_ms, and max_delay_ms from config.
    #[darling(default)]
    config_param: Option<String>,
    #[darling(default = "default_backoff")]
    backoff: String,
    #[darling(default = "default_delay")]
    delay: u64,
    #[darling(default)]
    max_delay: Option<u64>,
    #[darling(default)]
    jitter: bool,
    /// When true, checks RetryableError::is_retryable() before retrying.
    /// Also honors retry_after_ms() hint from errors.
    #[darling(default)]
    retryable: bool,
    /// Custom name for logging (defaults to function name).
    #[darling(default)]
    name: Option<String>,
}

fn default_max_attempts() -> usize {
    3
}

fn default_backoff() -> String {
    "exponential".to_string()
}

fn default_delay() -> u64 {
    100
}

pub fn impl_retry(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = RetryArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let func_name = &func.sig.ident;
    let name_storage;
    let operation_name = match &args.name {
        Some(name) => name.as_str(),
        None => {
            name_storage = func_name.to_string();
            &name_storage
        }
    };

    let use_retryable = args.retryable;

    // Determine if we're using config_param for all settings
    let use_config = args.config_param.is_some();
    let config_ident = args.config_param.as_ref().map(|name| {
        syn::Ident::new(name, proc_macro2::Span::call_site())
    });

    // Generate max_attempts expression
    let max_attempts_expr = if let Some(ref config) = config_ident {
        quote! { #config.max_attempts }
    } else if let Some(param_name) = &args.max_attempts_param {
        let param_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());
        quote! { #param_ident }
    } else {
        let max_attempts = args.max_attempts;
        quote! { #max_attempts }
    };

    // Generate delay expression
    let delay_expr = if let Some(ref config) = config_ident {
        quote! { #config.initial_delay_ms }
    } else {
        let delay = args.delay;
        quote! { #delay }
    };

    // Generate max_delay expression
    let max_delay_expr = if let Some(ref config) = config_ident {
        quote! { #config.max_delay_ms }
    } else if let Some(max_delay) = args.max_delay {
        quote! { #max_delay }
    } else {
        quote! { 10000_u64 }  // default max_delay
    };

    let visibility = &func.vis;
    let sig = &func.sig;
    let attrs = &func.attrs;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    // Generate backoff delay calculation (using expressions for config support)
    let backoff_logic = if use_config {
        // When using config, always use exponential backoff with config values
        quote! {
            ::std::time::Duration::from_millis(#delay_expr * (2_u64.pow(__retry_attempt as u32)))
        }
    } else {
        match args.backoff.as_str() {
            "fixed" => quote! {
                ::std::time::Duration::from_millis(#delay_expr)
            },
            "exponential" => quote! {
                ::std::time::Duration::from_millis(#delay_expr * (2_u64.pow(__retry_attempt as u32)))
            },
            "fibonacci" => quote! {
                {
                    fn __fib(n: u32) -> u64 {
                        match n {
                            0 => 0,
                            1 => 1,
                            n => __fib(n - 1) + __fib(n - 2),
                        }
                    }
                    ::std::time::Duration::from_millis(#delay_expr * __fib(__retry_attempt as u32))
                }
            },
            _ => quote! {
                ::std::time::Duration::from_millis(#delay_expr)
            },
        }
    };

    // Apply max_delay cap
    let delay_calc = if use_config {
        // Config mode: always apply cap using config value
        quote! {
            {
                let __calculated = #backoff_logic;
                let __max = ::std::time::Duration::from_millis(#max_delay_expr);
                if __calculated > __max { __max } else { __calculated }
            }
        }
    } else if args.max_delay.is_some() {
        quote! {
            {
                let __calculated = #backoff_logic;
                let __max = ::std::time::Duration::from_millis(#max_delay_expr);
                if __calculated > __max { __max } else { __calculated }
            }
        }
    } else {
        backoff_logic
    };

    // Add jitter if requested
    let final_delay = if args.jitter {
        quote! {
            {
                let __base_delay = #delay_calc;
                let __jitter_ms = (::std::time::SystemTime::now()
                    .duration_since(::std::time::UNIX_EPOCH)
                    .unwrap()
                    .subsec_nanos() % 100) as u64;
                __base_delay + ::std::time::Duration::from_millis(__jitter_ms)
            }
        }
    } else {
        delay_calc
    };

    // Generate the retry loop based on async/sync and retryable flag
    let wrapped = if is_async && use_retryable {
        // Async with RetryableError support
        quote! {
            #(#attrs)*
            #visibility #sig {
                use ::rustboot_error::RetryableError;

                let mut __retry_attempt: usize = 0;

                loop {
                    match (async #block).await {
                        Ok(__value) => return Ok(__value),
                        Err(__error) => {
                            __retry_attempt += 1;

                            // Check if error is retryable
                            if !__error.is_retryable() {
                                ::tracing::debug!(
                                    operation = #operation_name,
                                    "Non-retryable error, not retrying"
                                );
                                return Err(__error);
                            }

                            if __retry_attempt >= #max_attempts_expr {
                                ::tracing::warn!(
                                    operation = #operation_name,
                                    attempts = __retry_attempt,
                                    "Max retries exceeded"
                                );
                                return Err(__error);
                            }

                            // Use error's retry_after_ms hint if available, else use backoff
                            let __delay = __error.retry_after_ms()
                                .map(::std::time::Duration::from_millis)
                                .unwrap_or_else(|| #final_delay);

                            ::tracing::info!(
                                operation = #operation_name,
                                attempt = __retry_attempt,
                                max_attempts = #max_attempts_expr,
                                delay_ms = __delay.as_millis() as u64,
                                "Retrying after error"
                            );

                            ::tokio::time::sleep(__delay).await;
                        }
                    }
                }
            }
        }
    } else if is_async {
        // Async without RetryableError (retry on any error)
        quote! {
            #(#attrs)*
            #visibility #sig {
                let mut __retry_attempt: usize = 0;

                loop {
                    match (async #block).await {
                        Ok(__value) => return Ok(__value),
                        Err(__error) => {
                            __retry_attempt += 1;

                            if __retry_attempt >= #max_attempts_expr {
                                return Err(__error);
                            }

                            let __delay = #final_delay;
                            ::tokio::time::sleep(__delay).await;
                        }
                    }
                }
            }
        }
    } else if use_retryable {
        // Sync with RetryableError support
        quote! {
            #(#attrs)*
            #visibility #sig {
                use ::rustboot_error::RetryableError;

                let mut __retry_attempt: usize = 0;

                loop {
                    match (|| #block)() {
                        Ok(__value) => return Ok(__value),
                        Err(__error) => {
                            __retry_attempt += 1;

                            if !__error.is_retryable() {
                                return Err(__error);
                            }

                            if __retry_attempt >= #max_attempts_expr {
                                return Err(__error);
                            }

                            let __delay = __error.retry_after_ms()
                                .map(::std::time::Duration::from_millis)
                                .unwrap_or_else(|| #final_delay);

                            ::std::thread::sleep(__delay);
                        }
                    }
                }
            }
        }
    } else {
        // Sync without RetryableError (retry on any error)
        quote! {
            #(#attrs)*
            #visibility #sig {
                let mut __retry_attempt: usize = 0;

                loop {
                    match (|| #block)() {
                        Ok(__value) => return Ok(__value),
                        Err(__error) => {
                            __retry_attempt += 1;

                            if __retry_attempt >= #max_attempts_expr {
                                return Err(__error);
                            }

                            let __delay = #final_delay;
                            ::std::thread::sleep(__delay);
                        }
                    }
                }
            }
        }
    };

    Ok(wrapped)
}
