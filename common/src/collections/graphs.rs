//! Node-based graphs, for use in procedural and visual scripting and generation
//! tasks.
//!
//! The core type of this module is the [`Graph`], which contains a set of
//! [`Node`]s and the links between them. Each node has a unique identifier
//! [`NodeId`] , and can contain any type of data. Nodes are linked via their
//! [`GraphPort`]s, which are identified by a [`PortId`].
//!
//! The creation and editing of [`Graph`]s is a first-class concept in the
//! Surreal editor, and there are a variety of tools and UI to support this
//! process.
//!
//! Graphs can be serialized/deserialized directly via Serde to any desired
//! format.

use std::borrow::Cow;

crate::impl_rid!(NodeId);
crate::impl_rid!(PortId);

/// A managed graph of [`GraphNode`]s.
///
/// Nodes and ports are stored in an adjacency list via [`SlotMap`].
/// Each node is capable of persisting arbitrary data of type `D`.
///
/// Graphs are serializable for persistence.
#[derive(Default)]
pub struct Graph<D> {
  _phantom: std::marker::PhantomData<D>,
}

/// A single node in a [`Graph`].
pub struct GraphNode<D> {
  _phantom: std::marker::PhantomData<D>,
}

#[allow(unused_variables)]
impl<D> Graph<D> {
  /// Adds a new [`GraphNode`] to the graph.
  pub fn add_node(&mut self, label: impl Into<Cow<'static, str>>, user_data: D) {
    todo!()
  }

  /// Removes an existing [`GraphNode`] from the graph.
  pub fn remove_node(&mut self, node_id: NodeId) {
    todo!()
  }

  /// Adds a new connection to the graph.
  pub fn add_connection(&mut self, from: PortId, to: PortId) {
    todo!()
  }

  /// Removes an existing connection from the graph.
  pub fn remove_connection(&mut self, from: PortId, to: PortId) {
    todo!()
  }
}
