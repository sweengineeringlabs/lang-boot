use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

pub fn impl_validate(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Extract field information
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Validate can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Validate can only be derived for structs",
            ))
        }
    };

    // Generate validation logic for each field
    let validations = fields.iter().filter_map(|f| {
        let field_name = &f.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        // Check for validate attributes
        let validate_attrs: Vec<_> = f.attrs.iter()
            .filter(|attr| attr.path().is_ident("validate"))
            .collect();

        if validate_attrs.is_empty() {
            return None;
        }

        let mut checks = Vec::new();

        for attr in validate_attrs {
            // Parse validate attribute using Syn 2.0 API
            let _ = attr.parse_nested_meta(|meta| {
                // Check for #[validate(email)]
                if meta.path.is_ident("email") {
                    checks.push(quote! {
                        if !self.#field_name.contains('@') {
                            errors.push(format!("{}: must be a valid email address", #field_name_str));
                        }
                    });
                    return Ok(());
                }

                // Check for #[validate(length(min = X, max = Y))]
                if meta.path.is_ident("length") {
                    let mut min: Option<usize> = None;
                    let mut max: Option<usize> = None;

                    meta.parse_nested_meta(|nested| {
                        if nested.path.is_ident("min") {
                            min = Some(nested.value()?.parse::<syn::LitInt>()?.base10_parse()?);
                        } else if nested.path.is_ident("max") {
                            max = Some(nested.value()?.parse::<syn::LitInt>()?.base10_parse()?);
                        }
                        Ok(())
                    })?;

                    if let Some(min_val) = min {
                        checks.push(quote! {
                            if self.#field_name.len() < #min_val {
                                errors.push(format!("{}: must be at least {} characters", #field_name_str, #min_val));
                            }
                        });
                    }

                    if let Some(max_val) = max {
                        checks.push(quote! {
                            if self.#field_name.len() > #max_val {
                                errors.push(format!("{}: must be at most {} characters", #field_name_str, #max_val));
                            }
                        });
                    }
                    return Ok(());
                }

                // Check for #[validate(range(min = X, max = Y))]
                if meta.path.is_ident("range") {
                    let mut min: Option<i64> = None;
                    let mut max: Option<i64> = None;

                    meta.parse_nested_meta(|nested| {
                        if nested.path.is_ident("min") {
                            min = Some(nested.value()?.parse::<syn::LitInt>()?.base10_parse()?);
                        } else if nested.path.is_ident("max") {
                            max = Some(nested.value()?.parse::<syn::LitInt>()?.base10_parse()?);
                        }
                        Ok(())
                    })?;

                    if let Some(min_val) = min {
                        checks.push(quote! {
                            if (self.#field_name as i64) < #min_val {
                                errors.push(format!("{}: must be at least {}", #field_name_str, #min_val));
                            }
                        });
                    }

                    if let Some(max_val) = max {
                        checks.push(quote! {
                            if (self.#field_name as i64) > #max_val {
                                errors.push(format!("{}: must be at most {}", #field_name_str, #max_val));
                            }
                        });
                    }
                    return Ok(());
                }

                Ok(())
            }).ok();
        }

        if checks.is_empty() {
            None
        } else {
            Some(quote! {
                #(#checks)*
            })
        }
    });

    // Generate the implementation
    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn validate(&self) -> Result<(), Vec<String>> {
                let mut errors = Vec::new();

                #(#validations)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    Ok(expanded)
}
