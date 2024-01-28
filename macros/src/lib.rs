//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod binary;
mod component;
mod profiling;
mod reflect;
mod singleton;
mod vertex;

/// Derives the `FromBinary` and `ToBinary` traits for a type.
#[proc_macro_derive(Binary)]
pub fn derive_binary(input: TokenStream) -> TokenStream {
  binary::impl_binary(input)
}

/// Derives the `Component` trait for a type.
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
  component::impl_component(input)
}

/// Implements the Singleton pattern for a type.
#[proc_macro_derive(Singleton)]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
  singleton::impl_singleton(input)
}

/// Implements the `Reflect` trait for a type.
#[proc_macro_derive(Reflect)]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
  reflect::impl_reflect(input)
}

/// Instruments a function with profiling code.
#[proc_macro_attribute]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
  profiling::impl_profiling(item)
}

/// Derives the `Vertex` trait for a type.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}
