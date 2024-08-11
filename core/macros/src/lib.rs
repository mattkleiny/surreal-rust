//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod profiling;
mod singleton;
mod vertex;

/// Instruments a function with profiling code.
#[proc_macro_attribute]
pub fn profiling(_attr: TokenStream, item: TokenStream) -> TokenStream {
  profiling::impl_profiling(item)
}

/// Builds a singleton for a type.
#[proc_macro_derive(Singleton)]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
  singleton::impl_singleton(input)
}

/// Derives the `Vertex` trait for a type.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}
