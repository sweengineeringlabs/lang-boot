use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct FeatureFlagArgs {
    #[darling(rename = "flag")]
    flag_name: String,
}

pub fn impl_feature_flag(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = FeatureFlagArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let flag_name = &args.flag_name;

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    let wrapped = if is_async {
        quote! {
            #visibility #sig {
                // Check if feature flag is enabled
                if ::rustboot_config::is_feature_enabled(#flag_name).await {
                    (async #block).await
                } else {
                    // Return default or error
                    Err("Feature not enabled".into())
                }
            }
        }
    } else {
        quote! {
            #visibility #sig {
                // Check if feature flag is enabled
                if ::rustboot_config::is_feature_enabled(#flag_name) {
                    (|| #block)()
                } else {
                    // Return default or error
                    Err("Feature not enabled".into())
                }
            }
        }
    };

    Ok(wrapped)
}
