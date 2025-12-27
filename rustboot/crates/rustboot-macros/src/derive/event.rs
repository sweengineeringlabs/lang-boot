use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

pub fn impl_event(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Extract field information
    let _fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Event can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Event can only be derived for structs",
            ))
        }
    };

    let event_name_str = name.to_string();

    // Generate the implementation
    let expanded = quote! {
        impl #impl_generics ::rustboot_messaging::Event for #name #ty_generics #where_clause {
            fn event_type(&self) -> &str {
                #event_name_str
            }

            fn event_version(&self) -> &str {
                "1.0"
            }

            fn to_message(&self) -> ::rustboot_messaging::Message {
                ::rustboot_messaging::Message {
                    topic: self.event_type().to_string(),
                    payload: ::rustboot_serialization::serialize_json(self)
                        .expect("Failed to serialize event"),
                    metadata: ::std::collections::HashMap::new(),
                }
            }
        }

        // Automatically derive Serialize and Deserialize
        impl #impl_generics ::serde::Serialize for #name #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(#event_name_str, 2)?;
                // Serialize fields here
                state.end()
            }
        }
    };

    Ok(expanded)
}
