use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_trace(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let fields = match input.data {
    syn::Data::Struct(ref data) => match data.fields {
      syn::Fields::Named(ref fields) => fields.named.iter().map(|f| &f.ident),
      _ => unimplemented!(),
    },
    _ => unimplemented!(),
  };

  let expanded = quote! {
    unsafe impl Trace for #ident {
      fn trace(&self, context: &mut TraceContext) {
        #(
          self.#fields.trace(context);
        )*
      }
    }
  };

  expanded.into()
}
