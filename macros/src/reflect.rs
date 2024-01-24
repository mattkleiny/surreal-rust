use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_reflect(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  // parse the fields of the struct
  let fields: Vec<_> = match input.data {
    syn::Data::Struct(ref data_struct) => match &data_struct.fields {
      syn::Fields::Named(fields) => fields.named.iter(),
      syn::Fields::Unnamed(fields) => fields.unnamed.iter(),
      _ => panic!("Only structs are supported"),
    },
    _ => panic!("Only structs are supported"),
  }
  .map(|field| {
    let name = field.ident.as_ref().unwrap();
    let ty = &field.ty;

    quote! {
      name: stringify!(#name),
      kind: stringify!(#ty),
    }
  })
  .collect();

  let expanded = quote! {
    /// Implements reflection for the given type.
    impl Reflect for #name {
      fn properties(&self) -> Vec<Property> {
        let mut properties = Vec::new();

        #(
          properties.push(Property { #fields });
        )*

        properties
      }
     }
  };

  expanded.into()
}
