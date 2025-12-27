//! OpenAPI schema derive macro.
//!
//! Automatically generates OpenAPI schema implementations for structs and enums.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

pub fn impl_openapi_schema(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let name_str = name.to_string();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let schema_impl = match &input.data {
        Data::Struct(data) => generate_struct_schema(name, &data.fields)?,
        Data::Enum(data) => generate_enum_schema(name, &data.variants)?,
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                name,
                "OpenApiSchema cannot be derived for unions",
            ))
        }
    };

    let expanded = quote! {
        impl #impl_generics dev_engineeringlabs_rustboot_openapi::SchemaGenerator for #name #ty_generics #where_clause {
            fn schema() -> dev_engineeringlabs_rustboot_openapi::Schema {
                #schema_impl
            }

            fn schema_name() -> Option<String> {
                Some(#name_str.to_string())
            }
        }
    };

    Ok(expanded)
}

fn generate_struct_schema(
    name: &syn::Ident,
    fields: &Fields,
) -> Result<TokenStream> {
    match fields {
        Fields::Named(fields) => {
            let mut properties = Vec::new();
            let mut required = Vec::new();

            for field in &fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_name_str = field_name.to_string();
                let field_type = &field.ty;

                properties.push(quote! {
                    properties.insert(
                        #field_name_str.to_string(),
                        <#field_type as dev_engineeringlabs_rustboot_openapi::SchemaGenerator>::schema()
                    );
                });

                // Check if field is Option - if not, it's required
                let type_str = quote!(#field_type).to_string();
                if !type_str.contains("Option") {
                    required.push(quote! {
                        required.push(#field_name_str.to_string());
                    });
                }
            }

            Ok(quote! {
                {
                    let mut properties = std::collections::HashMap::new();
                    let mut required = Vec::new();

                    #(#properties)*
                    #(#required)*

                    dev_engineeringlabs_rustboot_openapi::Schema::Object(
                        dev_engineeringlabs_rustboot_openapi::spec::SchemaObject {
                            schema_type: Some("object".to_string()),
                            format: None,
                            description: None,
                            nullable: None,
                            properties,
                            required,
                            items: None,
                            enum_values: Vec::new(),
                            default: None,
                            example: None,
                            all_of: Vec::new(),
                            one_of: Vec::new(),
                            any_of: Vec::new(),
                        }
                    )
                }
            })
        }
        Fields::Unnamed(_) => {
            Err(syn::Error::new_spanned(
                name,
                "OpenApiSchema does not support tuple structs yet",
            ))
        }
        Fields::Unit => {
            Ok(quote! {
                dev_engineeringlabs_rustboot_openapi::Schema::Object(
                    dev_engineeringlabs_rustboot_openapi::spec::SchemaObject {
                        schema_type: Some("object".to_string()),
                        format: None,
                        description: None,
                        nullable: None,
                        properties: std::collections::HashMap::new(),
                        required: Vec::new(),
                        items: None,
                        enum_values: Vec::new(),
                        default: None,
                        example: None,
                        all_of: Vec::new(),
                        one_of: Vec::new(),
                        any_of: Vec::new(),
                    }
                )
            })
        }
    }
}

fn generate_enum_schema(
    _name: &syn::Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Result<TokenStream> {
    let mut enum_values = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();

        match &variant.fields {
            Fields::Unit => {
                enum_values.push(quote! {
                    enum_values.push(serde_json::json!(#variant_str));
                });
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "OpenApiSchema only supports unit variants for enums",
                ))
            }
        }
    }

    Ok(quote! {
        {
            let mut enum_values = Vec::new();
            #(#enum_values)*

            dev_engineeringlabs_rustboot_openapi::Schema::Object(
                dev_engineeringlabs_rustboot_openapi::spec::SchemaObject {
                    schema_type: Some("string".to_string()),
                    format: None,
                    description: None,
                    nullable: None,
                    properties: std::collections::HashMap::new(),
                    required: Vec::new(),
                    items: None,
                    enum_values,
                    default: None,
                    example: None,
                    all_of: Vec::new(),
                    one_of: Vec::new(),
                    any_of: Vec::new(),
                }
            )
        }
    })
}
