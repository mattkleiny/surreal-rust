use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_singleton_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  (quote! {
    use surreal::utilities::{Singleton, UnsafeLazyCell};

    impl Singleton for #name {
      fn instance() -> &'static mut Self {
        static mut INSTANCE: UnsafeLazyCell<#name> = UnsafeLazyCell::new();

        unsafe { &mut INSTANCE }
      }
    }
  })
  .into()
}
