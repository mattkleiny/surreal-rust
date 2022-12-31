//! Macros and compiler magic for the Surreal project.

use proc_macro::TokenStream;

mod object;
mod rsx;
mod vertex;

/// Builds an [`Object`] trait implementation for the associated struct.
#[proc_macro_derive(Object)]
pub fn derive_object(input: TokenStream) -> TokenStream {
  object::impl_object_trait(input)
}

/// Builds a [`Vertex`] trait implementation for the associated struct.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}

/// Builds a tree of `egui` node constructions based on a React-like markup language.
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
  rsx::impl_rsx_macro(input)
}
