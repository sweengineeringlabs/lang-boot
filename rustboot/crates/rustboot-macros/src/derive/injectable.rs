use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

pub fn impl_injectable(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Extract field information
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Injectable can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Injectable can only be derived for structs",
            ))
        }
    };

    // Generate field initialization from container
    let field_inits = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        
        quote! {
            #field_name: container.resolve::<#field_type>()?,
        }
    });

    // Generate the implementation
    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Create new instance from DI container
            pub fn from_container(
                container: &::std::sync::Arc<dyn ::rustboot_di::Container>
            ) -> ::std::result::Result<Self, ::rustboot_di::DiError> {
                Ok(Self {
                    #(#field_inits)*
                })
            }

            /// Register this type in the DI container
            pub fn register(
                container: &mut dyn ::rustboot_di::Container
            ) -> ::std::result::Result<(), ::rustboot_di::DiError> {
                container.register_factory(
                    |c| ::std::sync::Arc::new(Self::from_container(c)?)
                )
            }
        }
    };

    Ok(expanded)
}
