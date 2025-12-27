use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{Data, DeriveInput, Fields, Result, Type, Expr, Lit};

/// Configuration parsed from #[builder(...)] attributes
struct BuilderConfig {
    /// Generate with_* methods on the struct itself (fluent pattern)
    fluent: bool,
}

impl Default for BuilderConfig {
    fn default() -> Self {
        Self { fluent: false }
    }
}

fn parse_builder_config(input: &DeriveInput) -> BuilderConfig {
    let mut config = BuilderConfig::default();

    for attr in &input.attrs {
        if attr.path().is_ident("builder") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("fluent") {
                    config.fluent = true;
                }
                Ok(())
            });
        }
    }

    config
}

/// Check if a type is Option<T> and extract T
fn extract_option_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return Some(inner);
                    }
                }
            }
        }
    }
    None
}

/// Parse #[builder(default = "expr")] on fields
fn parse_field_default(field: &syn::Field) -> Option<proc_macro2::TokenStream> {
    for attr in &field.attrs {
        if attr.path().is_ident("builder") {
            let mut default_expr = None;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    if let Ok(value) = meta.value() {
                        if let Ok(Expr::Lit(expr_lit)) = value.parse::<Expr>() {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                let expr: proc_macro2::TokenStream = lit_str.value().parse().unwrap_or_default();
                                default_expr = Some(expr);
                            }
                        }
                    }
                }
                Ok(())
            });
            if default_expr.is_some() {
                return default_expr;
            }
        }
    }
    None
}

pub fn impl_builder(input: DeriveInput) -> Result<TokenStream> {
    let config = parse_builder_config(&input);

    if config.fluent {
        impl_fluent_builder(input)
    } else {
        impl_standard_builder(input)
    }
}

/// Generate with_* methods directly on the struct (fluent pattern)
///
/// Usage:
/// ```ignore
/// #[derive(Builder)]
/// #[builder(fluent)]
/// struct Config {
///     name: String,
///     #[builder(default = "None")]
///     timeout: Option<u64>,
/// }
///
/// // Generates:
/// impl Config {
///     pub fn with_name(mut self, value: String) -> Self { ... }
///     pub fn with_timeout(mut self, value: u64) -> Self { ... }  // unwraps Option
/// }
/// ```
fn impl_fluent_builder(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Builder can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Builder can only be derived for structs",
            ))
        }
    };

    // Generate with_* methods
    let with_methods = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        let method_name = format_ident!("with_{}", field_name.as_ref().unwrap());

        // If field is Option<T>, generate method that takes T and wraps in Some
        if let Some(inner_type) = extract_option_inner(field_type) {
            quote! {
                /// Set the #field_name field
                pub fn #method_name(mut self, value: #inner_type) -> Self {
                    self.#field_name = ::std::option::Option::Some(value);
                    self
                }
            }
        } else {
            quote! {
                /// Set the #field_name field
                pub fn #method_name(mut self, value: #field_type) -> Self {
                    self.#field_name = value;
                    self
                }
            }
        }
    });

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#with_methods)*
        }
    };

    Ok(expanded)
}

/// Generate a separate Builder struct (standard pattern)
fn impl_standard_builder(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Builder can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Builder can only be derived for structs",
            ))
        }
    };

    // Generate builder struct fields (all Option<T>)
    let builder_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            #field_name: ::std::option::Option<#field_type>,
        }
    });

    // Generate setter methods
    let setters = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            pub fn #field_name(mut self, value: #field_type) -> Self {
                self.#field_name = ::std::option::Option::Some(value);
                self
            }
        }
    });

    // Generate field initialization in build()
    let field_inits = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        // Check for default value
        if let Some(default_expr) = parse_field_default(f) {
            quote! {
                #field_name: self.#field_name.unwrap_or_else(|| #default_expr),
            }
        } else {
            quote! {
                #field_name: self.#field_name
                    .ok_or_else(|| format!("Field '{}' is required", #field_name_str))?,
            }
        }
    });

    let expanded = quote! {
        #[derive(Default)]
        pub struct #builder_name #impl_generics #where_clause {
            #(#builder_fields)*
        }

        impl #impl_generics #builder_name #ty_generics #where_clause {
            pub fn new() -> Self {
                Self::default()
            }

            #(#setters)*

            pub fn build(self) -> ::std::result::Result<#name #ty_generics, String> {
                Ok(#name {
                    #(#field_inits)*
                })
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn builder() -> #builder_name #ty_generics {
                #builder_name::new()
            }
        }
    };

    Ok(expanded)
}
