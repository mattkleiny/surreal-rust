//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod component;
mod game;
mod profiling;
mod singleton;
mod vertex;

/// Derives the `Component` trait for a type.
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
  component::impl_component(input)
}

/// Implements the `Game` trait for a type.
#[proc_macro_derive(Game)]
pub fn derive_game(input: TokenStream) -> TokenStream {
  game::impl_game(input)
}

/// Implements the Singleton pattern for a type.
#[proc_macro_derive(Singleton)]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
  singleton::impl_singleton(input)
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
