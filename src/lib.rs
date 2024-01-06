//! More procedural macros for [`derive_builder`](https://docs.rs/derive_builder).

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};

/// Derive macro `From<T> for TBuilder`.
/// Automatically implements the conversion back from `T` into `TBuilder`.
/// Furthermore, `T::builder()` and `T::configure(self)` methods are also implemented for `T`.
///
/// # Panics
///
/// Panics if the input is not a struct with named fields annotated as `#[derive(Builder)]`.
#[allow(non_snake_case)]
#[proc_macro_derive(IntoBuilder)]
pub fn derive_IntoBuilder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let struct_builder_ident = Ident::new(&format!("{struct_ident}Builder"), struct_ident.span());

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Trait Into<{struct_ident}Builder> cannot be derived for {struct_ident} because it is not a struct with named fields"),
    };
    let field_names = fields.iter().map(|field| &field.ident);

    let expanded = quote! {
        #[automatically_derived]
        impl From<#struct_ident> for #struct_builder_ident {
            fn from(value: #struct_ident) -> Self {
                Self {
                    #(
                        #field_names: Some(value.#field_names),
                    )*
                }
            }
        }

        #[automatically_derived]
        impl #struct_ident {
            pub fn configure(self) -> #struct_builder_ident {
                self.into()
            }

            pub fn builder() -> #struct_builder_ident {
                #struct_builder_ident::default()
            }
        }
    };

    TokenStream::from(expanded)
}
