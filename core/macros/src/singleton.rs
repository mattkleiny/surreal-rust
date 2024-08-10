use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_singleton(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let expanded = quote! {
    impl #ident {
      pub fn instance() -> &'static mut #ident {
        static mut INSTANCE: surreal::common::UnsafeSingleton<#ident> = surreal::common::UnsafeSingleton::default();

        unsafe { &mut INSTANCE }
      }
    }
  };

  expanded.into()
}
