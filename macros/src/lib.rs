//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod profiling;
mod singleton;
mod vertex;

/// Builds a [`Singleton`] trait implementation for the associated struct.
#[proc_macro_derive(Singleton)]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
  singleton::impl_singleton_trait(input)
}

/// Builds a [`Vertex`] trait implementation for the associated struct.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}

/// Marks a function for profiling with the profiling module
#[proc_macro_attribute]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
  profiling::impl_profiling(item)
}
