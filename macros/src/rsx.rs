use proc_macro::TokenStream;

use quote::ToTokens;
use syn::parse::Parse;
use syn::parse_macro_input;

pub fn impl_rsx_macro(input: TokenStream) -> TokenStream {
  let root = parse_macro_input!(input as RsxRootNode);

  TokenStream::from(root.into_token_stream())
}

struct RsxRootNode {}

impl Parse for RsxRootNode {
  fn parse(_input: syn::parse::ParseStream) -> syn::Result<Self> {
    // TODO: implement me

    Ok(Self {})
  }
}

impl ToTokens for RsxRootNode {
  fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
    // TODO: implement me
  }
}
