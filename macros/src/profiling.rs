use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, parse_quote};

pub fn impl_profiling(item: TokenStream) -> TokenStream {
  let mut function = parse_macro_input!(item as syn::ItemFn);
  let block = function.block;

  function.block = Box::<syn::Block>::new(parse_quote! {
    {
      surreal::diagnostics::profiling::profile_function!();
      #block
    }
  });

  (quote! {
    #function
  })
  .into()
}
