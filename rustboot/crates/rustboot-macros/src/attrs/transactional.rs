use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

pub fn impl_transactional(_args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let func = &input;
    let func_name = &func.sig.ident;
    let _func_name_str = func_name.to_string();

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let is_async = sig.asyncness.is_some();

    let wrapped = if is_async {
        quote! {
            #visibility #sig {
                // Begin transaction
                let tx = ::rustboot_database::begin_transaction().await?;

                // Execute function within transaction
                let result = async {
                    #block
                }.await;

                match result {
                    Ok(value) => {
                        // Commit on success
                        ::rustboot_database::commit_transaction(tx).await?;
                        Ok(value)
                    }
                    Err(err) => {
                        // Rollback on error
                        ::rustboot_database::rollback_transaction(tx).await?;
                        Err(err)
                    }
                }
            }
        }
    } else {
        quote! {
            #visibility #sig {
                // Begin transaction
                let tx = ::rustboot_database::begin_transaction()?;

                // Execute function within transaction
                let result = (|| #block)();

                match result {
                    Ok(value) => {
                        // Commit on success
                        ::rustboot_database::commit_transaction(tx)?;
                        Ok(value)
                    }
                    Err(err) => {
                        // Rollback on error
                        ::rustboot_database::rollback_transaction(tx)?;
                        Err(err)
                    }
                }
            }
        }
    };

    Ok(wrapped)
}
