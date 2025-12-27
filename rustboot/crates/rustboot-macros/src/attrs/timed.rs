use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

pub fn impl_timed(_args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let func = &input;
    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    let wrapped = if is_async {
        quote! {
            #visibility #sig {
                let start = ::std::time::Instant::now();
                let result = (async #block).await;
                let duration = start.elapsed();

                ::rustboot_observability::metrics::record_duration(
                    #func_name_str,
                    duration
                );

                result
            }
        }
    } else {
        quote! {
            #visibility #sig {
                let start = ::std::time::Instant::now();
                let result = (|| #block)();
                let duration = start.elapsed();

                ::rustboot_observability::metrics::record_duration(
                    #func_name_str,
                    duration
                );

                result
            }
        }
    };

    Ok(wrapped)
}
