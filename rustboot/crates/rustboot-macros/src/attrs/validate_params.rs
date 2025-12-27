use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result, FnArg, Pat};

pub fn impl_validate_params(_args: TokenStream1, input: ItemFn) -> Result<TokenStream> {
    let func = &input;
    let visibility = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    // Extract parameters that have #[validate] attributes
    let validations: Vec<_> = sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            // Check for validate attribute
            let has_validate = pat_type.attrs.iter()
                .any(|attr| attr.path().is_ident("validate"));
            
            if has_validate {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = &pat_ident.ident;
                    let param_name_str = param_name.to_string();
                    
                    // Generate validation code
                    // For now, just check for basic validation
                    return Some(quote! {
                        if let Err(e) = ::rustboot_validation::validate_value(&#param_name) {
                            return Err(::rustboot_validation::ValidationError::new(
                                #param_name_str,
                                e.to_string()
                            ));
                        }
                    });
                }
            }
        }
        None
    }).collect();

    // If no validations, return unchanged
    if validations.is_empty() {
        return Ok(quote! { #input });
    }

    // Generate wrapped function with validation
    let wrapped = quote! {
        #visibility #sig {
            // Run all parameter validations
            #(#validations)*

            // Execute original function
            (|| #block)()
        }
    };

    Ok(wrapped)
}
