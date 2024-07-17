use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_serialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let expanded = quote! {
    impl Serialize for #ident {
      fn serialize(&self) -> Chunk {
        todo!()
      }
    }
  };

  expanded.into()
}

pub fn impl_deserialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let expanded = quote! {
    impl Deserialize for #ident {
      fn deserialize(chunk: Chunk) -> Self {
        todo!()
      }
    }
  };

  expanded.into()
}
