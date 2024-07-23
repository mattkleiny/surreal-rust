use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_uniform_set_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let fields = match input.data {
    syn::Data::Struct(ref data) => match data.fields {
      syn::Fields::Named(ref fields) => fields.named.iter().map(|f| &f.ident),
      _ => todo!(),
    },
    _ => todo!(),
  };

  let expanded = quote! {
    impl ToShaderUniformSet for #ident {
      fn apply_to(&self, set: &mut ShaderUniformSet) {
        #(
          set.set_uniform(stringify!(#fields), &self.#fields);
        )*
      }
    }
  };

  expanded.into()
}
