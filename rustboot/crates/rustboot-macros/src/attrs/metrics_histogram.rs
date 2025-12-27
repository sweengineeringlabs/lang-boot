use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct MetricsHistogramArgs {
    name: String,
}

pub fn impl_metrics_histogram(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = MetricsHistogramArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let metric_name = &args.name;

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

                ::rustboot_observability::metrics::record_histogram(
                    #metric_name,
                    duration.as_millis() as f64
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

                ::rustboot_observability::metrics::record_histogram(
                    #metric_name,
                    duration.as_millis() as f64
                );

                result
            }
        }
    };

    Ok(wrapped)
}
