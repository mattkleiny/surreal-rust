use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_singleton(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let expanded = quote! {
    impl Singleton for #ident {
      fn instance() -> &'static Self {
        static INSTANCE: std::sync::LazyLock<#ident> = std::sync::LazyLock::new(|| #ident::default());

        std::ops::Deref::deref(&INSTANCE)
      }
    }
  };

  expanded.into()
}
