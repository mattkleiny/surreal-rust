//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod assets;
mod formats;
mod objects;
mod profiling;
mod reflect;
mod vertex;

/// Instruments a function with profiling code.
#[proc_macro_attribute]
pub fn profiling(_attr: TokenStream, item: TokenStream) -> TokenStream {
  profiling::impl_profiling(item)
}

/// Derives the `Asset` trait for a type.
#[proc_macro_derive(Asset)]
pub fn derive_asset(input: TokenStream) -> TokenStream {
  assets::impl_asset(input)
}

/// Derives the `Reflect` trait for a type.
#[proc_macro_derive(Reflect)]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
  reflect::impl_reflect(input)
}

/// Derives the `Serialize` trait for a type.
#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
  formats::impl_serialize(input)
}

/// Derives the `Deserialize` trait for a type.
#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
  formats::impl_deserialize(input)
}

/// Derives the `Trace` trait for a type.
#[proc_macro_derive(Trace)]
pub fn derive_trace(input: TokenStream) -> TokenStream {
  objects::impl_trace(input)
}

/// Derives the `Vertex` trait for a type.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}
