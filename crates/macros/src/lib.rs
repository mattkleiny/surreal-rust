//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod profiling;
mod reflect;
mod serial;
mod singleton;
mod uniforms;
mod vertex;

/// Instruments a function with profiling code.
#[proc_macro_attribute]
pub fn profiling(_attr: TokenStream, item: TokenStream) -> TokenStream {
  profiling::impl_profiling(item)
}

/// Derives the `Reflect` trait for a type.
#[proc_macro_derive(Reflect)]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
  reflect::impl_reflect(input)
}

/// Derives the `Serialize` trait for a type.
#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
  serial::impl_serialize(input)
}

/// Derives the `Deserialize` trait for a type.
#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
  serial::impl_deserialize(input)
}

/// Derives the `Singleton` trait for a type.
#[proc_macro_derive(Singleton)]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
  singleton::impl_singleton(input)
}

/// Derives the `ToShaderUniformSet` trait for a type.
#[proc_macro_derive(ToShaderUniformSet, attributes(uniform))]
pub fn derive_uniform_set(input: TokenStream) -> TokenStream {
  uniforms::impl_uniform_set_trait(input)
}

/// Derives the `Vertex` trait for a type.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}
