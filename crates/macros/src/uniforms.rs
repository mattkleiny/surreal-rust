use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_uniform_set_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let expanded = quote! {
    impl ToShaderUniformSet for #ident {
      fn apply_to(&self, set: &mut ShaderUniformSet) {
        // TODO: implement me
      }
    }
  };

  expanded.into()
}
