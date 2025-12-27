use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

/// Generate a workflow builder pattern.
/// 
/// Unlike regular Builder, WorkflowBuilder is for builders that:
/// - Accumulate items in a collection
/// - Have custom methods beyond simple setters
/// - Build into a different type than the builder
///
/// Example:
/// ```rust
/// #[derive(WorkflowBuilder)]
/// #[workflow(builds = "CompositeValidator<String>", accumulates = "Box<dyn Validator<String>>")]
/// struct StringValidationBuilder {
///     field: String,
///     validators: Vec<Box<dyn Validator<String>>>,
/// }
/// ```
pub fn impl_workflow_builder(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    
    // Extract workflow attributes
    let builds_type = extract_builds_type(&input)?;
    let accumulate_field = extract_accumulate_field(&input)?;
    
    // Get all fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "WorkflowBuilder requires named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "WorkflowBuilder only works on structs",
            ))
        }
    };
    
    // Generate new() constructor
    let field_inits = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        
        // Special handling for Vec fields
        if is_vec_type(field_type) {
            quote! { #field_name: Vec::new() }
        } else {
            // For other fields, they need to be provided in new()
            quote! { #field_name }
        }
    });
    
    // Get constructor parameters (non-Vec fields)
    let ctor_params = fields.iter().filter_map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        
        if !is_vec_type(field_type) {
            Some(quote! { #field_name: impl Into<String> })
        } else {
            None
        }
    });
    
    let ctor_param_names = fields.iter().filter_map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        
        if !is_vec_type(field_type) {
            Some(field_name)
        } else {
            None
        }
    });
    
    // Generate implementation
    let expanded = quote! {
        impl #name {
            /// Create a new workflow builder
            pub fn new(#(#ctor_params),*) -> Self {
                Self {
                    #(#field_inits),*
                }
            }
            
            // Custom methods should be defined by user
            // build() should also be defined by user
        }
        
        impl Default for #name {
            fn default() -> Self {
                Self::new(String::new())
            }
        }
    };
    
    Ok(expanded)
}

/// Check if a type is Vec<T>
fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// Extract the type this builder builds into
fn extract_builds_type(input: &DeriveInput) -> Result<TokenStream> {
    // For now, we'll require users to specify this via attribute
    // #[workflow(builds = "TargetType")]
    Ok(quote! { () }) // Placeholder
}

/// Extract the field that accumulates items
fn extract_accumulate_field(input: &DeriveInput) -> Result<syn::Ident> {
    // For now, assume it's "validators" or similar
    Ok(syn::Ident::new("validators", proc_macro2::Span::call_site()))
}
