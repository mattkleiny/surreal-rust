use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_binary(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

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
