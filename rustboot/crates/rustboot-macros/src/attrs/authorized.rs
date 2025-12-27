use darling::FromMeta;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

#[derive(Debug, FromMeta)]
struct AuthorizedArgs {
    #[darling(default)]
    role: Option<String>,
    #[darling(default)]
    permission: Option<String>,
    #[darling(default)]
    require_all: Vec<syn::LitStr>,
}

pub fn impl_authorized(args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(args.into())?;
    let args = AuthorizedArgs::from_list(&attr_args)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let func = &input;
    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    // Generate authorization check
    let auth_check = if let Some(role) = &args.role {
        quote! {
            ::rustboot_security::authz::check_role(#role).await?;
        }
    } else if let Some(perm) = &args.permission {
        quote! {
            ::rustboot_security::authz::check_permission(#perm).await?;
        }
    } else if !args.require_all.is_empty() {
        let perms = args.require_all.iter().map(|lit| lit.value());
        quote! {
            ::rustboot_security::authz::check_all_permissions(&[#(#perms),*]).await?;
        }
    } else {
        quote! {
            ::rustboot_security::authz::check_authenticated().await?;
        }
    };

    let wrapped = if is_async {
        quote! {
            #visibility #sig {
                // Check authorization
                #auth_check

                // Execute original function
                (async #block).await
            }
        }
    } else {
        quote! {
            #visibility #sig {
                // Check authorization (blocking)
                #auth_check

                // Execute original function
                (|| #block)()
            }
        }
    };

    Ok(wrapped)
}
