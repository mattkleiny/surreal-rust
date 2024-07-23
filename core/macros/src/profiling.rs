use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

pub fn impl_profiling(item: TokenStream) -> TokenStream {
  let mut function = parse_macro_input!(item as syn::ItemFn);

  let ident = function.sig.ident.clone();
  let block = function.block;

  // rewrite the function to wrap the block in a profiling scope
  function.block = Box::new(parse_quote! {{
    common::profile_function!(stringify!(#ident));
    #block
  }});

  let expanded = quote! {
    #function
  };

  expanded.into()
}
