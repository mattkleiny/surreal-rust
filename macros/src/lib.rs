//! Macros and compiler magic for the Surreal project.

mod vertex;

use proc_macro::TokenStream;

/// Builds a [`Vertex`] trait implementation for the associated struct.
///
/// Defines a series of VertexDescriptors which define the layout of the vertex on the GPU.
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  vertex::impl_vertex_trait(input)
}
