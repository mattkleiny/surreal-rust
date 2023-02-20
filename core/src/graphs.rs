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

use slotmap::{SecondaryMap, SlotMap};

slotmap::new_key_type! {
  /// A [`slotmap::Key`] for [`GraphNode`]s.
  pub struct NodeId;

  /// A [`slotmap::Key`] for [`GraphPort`]s.
  pub struct PortId;
}

/// A managed graph of [`GraphNode`]s.
///
/// Nodes and ports are stored in an adjacency list via [`SlotMap`].
/// Each node is capable of persisting arbitrary data of type `D`.
///
/// Graphs are serializable for persistence.
#[derive(Serialize, Deserialize, Default)]
pub struct Graph<D = ()> {
  nodes: SlotMap<NodeId, GraphNode<D>>,
  ports: SlotMap<PortId, GraphPort<D>>,
  inputs: SlotMap<PortId, GraphInput<D>>,
  outputs: SlotMap<PortId, GraphOutput<D>>,
  connections: SecondaryMap<PortId, PortId>,
}

/// A single node in a [`Graph`].
#[derive(Serialize, Deserialize)]
pub struct GraphNode<D = ()> {
  id: NodeId,
  label: Cow<'static, str>,
  user_data: D,
}

/// A single port in a [`Graph`].
#[derive(Serialize, Deserialize)]
pub struct GraphPort<D> {
  id: PortId,
  label: Cow<'static, str>,
  value: D,
}

/// An input to a [`Graph`].
#[derive(Serialize, Deserialize, Default)]
pub struct GraphInput<D> {
  port: PortId,
  data: D,
}

/// An output from a [`Graph`].
#[derive(Serialize, Deserialize, Default)]
pub struct GraphOutput<D> {
  port: PortId,
  data: D,
}

impl<D> Graph<D> {
  /// Adds a new [`GraphNode`] to the graph.
  pub fn add_node(&mut self, label: impl Into<Cow<'static, str>>, user_data: D) {
    let label = label.into();

    self.nodes.insert_with_key(|id| GraphNode {
      id,
      label,
      user_data,
    });
  }

  /// Removes an existing [`GraphNode`] from the graph.
  pub fn remove_node(&mut self, node_id: NodeId) {
    if let Some(_node) = self.nodes.remove(node_id) {
      // TODO: remove any connections, too
    }
  }

  /// Iterates the [`GraphNode`]s of this graph.
  pub fn nodes(&self) -> impl Iterator<Item = &GraphNode<D>> {
    self.nodes.iter().map(|(_, node)| node)
  }

  /// Mutably iterates the [`GraphNode`]s of this graph.
  pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut GraphNode<D>> {
    self.nodes.iter_mut().map(|(_, node)| node)
  }
}

impl<'a, D> IntoIterator for &'a Graph<D> {
  type Item = &'a GraphNode<D>;
  type IntoIter = impl Iterator<Item = &'a GraphNode<D>>;

  fn into_iter(self) -> Self::IntoIter {
    self.nodes()
  }
}

impl<'a, D> IntoIterator for &'a mut Graph<D> {
  type Item = &'a mut GraphNode<D>;
  type IntoIter = impl Iterator<Item = &'a mut GraphNode<D>>;

  fn into_iter(self) -> Self::IntoIter {
    self.nodes_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::io::Serializable;

  #[test]
  fn graph_should_build_simple_connections() {
    let mut graph = Graph::<u32>::default();

    graph.add_node("Node 1", 1);
    graph.add_node("Node 2", 2);

    println!("{}", graph.to_json().unwrap());
  }
}
