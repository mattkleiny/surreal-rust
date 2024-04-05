use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_binary(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  // TODO: reflect over each of the fields in order
  // TODO: if the field has a ToBinary trait implementation, use it to serialize
  // TODO: make sure to serialize the fields in the same order as they are defined
  // TODO: serialize into a Vec<u8> and return it

  let expanded = quote! {
    impl crate::ToBinary for #name {
      #[inline]
      fn to_binary(&self) -> Vec<u8> {
        todo!()
      }
    }

    impl crate::FromBinary for #name {
      #[inline]
      fn from_binary(bytes: &[u8]) -> Self {
        todo!()
      }
    }
  };

  expanded.into()
}
