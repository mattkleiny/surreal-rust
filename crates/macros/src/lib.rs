extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(_item: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[proc_macro_attribute]
pub fn derive_vertex_attribute(_attr: TokenStream, _item: TokenStream) -> TokenStream {
  TokenStream::new()
}