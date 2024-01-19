use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

pub fn impl_profiling(item: TokenStream) -> TokenStream {
  let mut function = parse_macro_input!(item as syn::ItemFn);

  let name = function.sig.ident.clone();
  let block = function.block;

  // rewrite the function to wrap the block in a profiling scope
  function.block = Box::<syn::Block>::new(parse_quote! {
    {
      diagnostics::profile_function!(#name);
      #block
    }
  });

  let expanded = quote! {
    #function
  };

  expanded.into()
}
