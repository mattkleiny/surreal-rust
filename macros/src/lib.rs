//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod component;
mod profiling;
mod singleton;
mod vertex;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
  component::impl_component(input)
}

#[proc_macro_derive(Singleton)]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
  singleton::impl_singleton(input)
}

#[proc_macro_attribute]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
  profiling::impl_profiling(item)
}

#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}
