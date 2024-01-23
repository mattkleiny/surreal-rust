use std::{fmt::Debug, hash::Hash};

use crate::{impl_rid, Arena, Scalar};

impl_rid!(GraphNodeId);

/// An edge in a directed graph with a weight.
#[derive(Debug)]
pub struct GraphEdge<W: Scalar = f32> {
  pub from: GraphNodeId,
  pub to: GraphNodeId,
  pub weight: W,
}

// A graph of nodes and Graphedges stored in an adjacency list.
#[derive(Debug)]
pub struct Graph<N, W: Scalar = f32> {
  nodes: Arena<N>,
  edges: Vec<GraphEdge<W>>,
}

impl<N, W: Scalar> Default for Graph<N, W> {
  fn default() -> Self {
    Self {
      nodes: Arena::default(),
      edges: Vec::default(),
    }
  }
}

/// Represents a directed graph of nodes, with support for weighted edges.
pub trait DirectedGraph {
  /// The type of node in the graph.
  type Node;

  /// The type of weight on the edges.
  type Weight: Scalar;

  // node access
  fn node(&self, node: GraphNodeId) -> Option<&Self::Node>;
  fn node_mut(&mut self, node: GraphNodeId) -> Option<&mut Self::Node>;
  fn nodes(&self) -> impl Iterator<Item = &Self::Node>;
  fn add_node(&mut self, node: Self::Node) -> GraphNodeId;
  fn remove_node(&mut self, node: GraphNodeId) -> Option<Self::Node>;

  // edge access
  fn edges(&self) -> impl Iterator<Item = &GraphEdge<Self::Weight>>;
  fn edges_from(&self, node: GraphNodeId) -> impl Iterator<Item = &GraphEdge<Self::Weight>>;
  fn edges_to(&self, node: GraphNodeId) -> impl Iterator<Item = &GraphEdge<Self::Weight>>;
  fn add_edge(&mut self, from: GraphNodeId, to: GraphNodeId, weight: Self::Weight);
  fn remove_edge(&mut self, from: GraphNodeId, to: GraphNodeId);

  /// P
  fn to_dot(&self) -> String
  where
    Self::Node: Debug,
    Self::Weight: Debug,
  {
    // TODO: fix this up
    let mut dot = String::new();

    dot.push_str("digraph {\n");

    for node in self.nodes() {
      dot.push_str(&format!("  {:?};\n", node));
    }

    for edge in self.edges() {
      dot.push_str(&format!(
        "  {:?} -> {:?} [label={:?}];\n",
        edge.from, edge.to, edge.weight
      ));
    }

    dot.push_str("}\n");

    dot
  }
}

impl<N, W: Scalar> DirectedGraph for Graph<N, W> {
  type Node = N;
  type Weight = W;

  fn node(&self, node: GraphNodeId) -> Option<&Self::Node> {
    self.nodes.get(node.into())
  }

  fn node_mut(&mut self, node: GraphNodeId) -> Option<&mut Self::Node> {
    self.nodes.get_mut(node.into())
  }

  fn nodes(&self) -> impl Iterator<Item = &Self::Node> {
    self.nodes.iter()
  }

  fn add_node(&mut self, node: Self::Node) -> GraphNodeId {
    self.nodes.insert(node).into()
  }

  fn remove_node(&mut self, node: GraphNodeId) -> Option<Self::Node> {
    self.nodes.remove(node.into())
  }

  fn edges(&self) -> impl Iterator<Item = &GraphEdge<Self::Weight>> {
    self.edges.iter()
  }

  fn edges_from(&self, node: GraphNodeId) -> impl Iterator<Item = &GraphEdge<Self::Weight>> {
    self.edges.iter().filter(move |edge| edge.from == node)
  }

  fn edges_to(&self, node: GraphNodeId) -> impl Iterator<Item = &GraphEdge<Self::Weight>> {
    self.edges.iter().filter(move |edge| edge.to == node)
  }

  fn add_edge(&mut self, from: GraphNodeId, to: GraphNodeId, weight: Self::Weight) {
    self.edges.push(GraphEdge { from, to, weight });
  }

  fn remove_edge(&mut self, from: GraphNodeId, to: GraphNodeId) {
    self.edges.retain(|edge| edge.from != from || edge.to != to);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_graph_construction() {
    let mut graph = Graph::default();

    let node1 = graph.add_node("a");
    let node2 = graph.add_node("b");
    let node3 = graph.add_node("c");

    graph.add_edge(node1, node2, 1.0);
    graph.add_edge(node1, node3, 2.0);
    graph.add_edge(node2, node3, 3.0);

    assert_eq!(graph.nodes.len(), 3);
    assert_eq!(graph.edges.len(), 3);

    assert_eq!(graph.edges_from(node1).count(), 2);
    assert_eq!(graph.edges_from(node2).count(), 1);

    assert_eq!(graph.edges_to(node1).count(), 0);
    assert_eq!(graph.edges_to(node2).count(), 1);
    assert_eq!(graph.edges_to(node3).count(), 2);
  }
}
