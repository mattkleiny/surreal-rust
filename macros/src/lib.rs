//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod profiling;
mod vertex;

/// Marks a function for profiling with the profiling module
#[proc_macro_attribute]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
  profiling::impl_profiling(item)
}

/// Builds a [`Vertex`] trait implementation for the associated struct.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}
