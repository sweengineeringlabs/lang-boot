use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct TimeoutArgs {
    duration: u64,
}

pub fn impl_timeout(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = TimeoutArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let duration = args.duration;

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    if !is_async {
        return Err(syn::Error::new_spanned(
            func,
            "timeout macro only works with async functions",
        ));
    }

    let wrapped = quote! {
        #visibility #sig {
            use ::rustboot_resilience::timeout::with_timeout;
            
            let duration = ::std::time::Duration::from_millis(#duration);
            
            with_timeout(duration, async #block).await
        }
    };

    Ok(wrapped)
}
