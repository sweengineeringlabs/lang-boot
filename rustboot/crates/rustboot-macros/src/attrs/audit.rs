use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

pub fn impl_audit(_args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let func = &input;
    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();

    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    let wrapped = quote! {
        #visibility #sig {
            ::rustboot_security::audit::log_action(#func_name_str);
            (|| #block)()
        }
    };

    Ok(wrapped)
}
