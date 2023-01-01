use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_object_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  (quote! {
    impl surreal::utilities::Object for #name {
      #[inline(always)]
      fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
      }

      #[inline(always)]
      fn as_any(&self) -> &dyn std::any::Any {
        self
      }

      #[inline(always)]
      fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
      }

      #[inline(always)]
      fn get_type(&self) -> surreal::utilities::Type {
        surreal::utilities::Type::from(stringify!(#name))
      }
    }

    impl surreal::utilities::TypeOf for #name {
      #[inline(always)]
      fn type_of() -> surreal::utilities::Type {
        surreal::utilities::Type::from(stringify!(#name))
      }
    }
  })
  .into()
}
