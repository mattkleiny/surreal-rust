use std::{fmt::Debug, hash::Hash};

use crate::{impl_rid, Arena, NeighbourList, PathFindingGrid, Scalar};

impl_rid!(GraphNodeId, "Identifies a node in a graph.");

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

impl<N, W: Scalar> Graph<N, W> {
  /// Gets a reference to the node with the given id.
  pub fn node(&self, node: GraphNodeId) -> Option<&N> {
    self.nodes.get(node.into())
  }

  /// Gets a mutable reference to the node with the given id.
  pub fn node_mut(&mut self, node: GraphNodeId) -> Option<&mut N> {
    self.nodes.get_mut(node.into())
  }

  /// Iterates over the nodes in the graph.
  pub fn nodes(&self) -> impl Iterator<Item = &N> {
    self.nodes.iter()
  }

  /// Adds a new node to the graph.
  pub fn add_node(&mut self, node: N) -> GraphNodeId {
    self.nodes.insert(node).into()
  }

  /// Removes a node from the graph.
  pub fn remove_node(&mut self, node: GraphNodeId) -> Option<N> {
    self.nodes.remove(node.into())
  }

  /// Iterates over the edges in the graph.
  pub fn edges(&self) -> impl Iterator<Item = &GraphEdge<W>> {
    self.edges.iter()
  }

  /// Iterates over the edges that start at the given node.
  pub fn edges_from(&self, node: GraphNodeId) -> impl Iterator<Item = &GraphEdge<W>> {
    self.edges.iter().filter(move |edge| edge.from == node)
  }

  /// Iterates over the edges that end at the given node.
  pub fn edges_to(&self, node: GraphNodeId) -> impl Iterator<Item = &GraphEdge<W>> {
    self.edges.iter().filter(move |edge| edge.to == node)
  }

  /// Adds a new edge to the graph.
  pub fn add_edge(&mut self, from: GraphNodeId, to: GraphNodeId, weight: W) {
    self.edges.push(GraphEdge { from, to, weight });
  }

  /// Removes an edge from the graph.
  pub fn remove_edge(&mut self, from: GraphNodeId, to: GraphNodeId) {
    self.edges.retain(|edge| edge.from != from || edge.to != to);
  }
}

/// Allows a directed graph to be used for path-finding.
impl<N> PathFindingGrid<GraphNodeId> for Graph<N, f32> {
  /// Gets the pathing cost between the given two node.
  fn get_cost(&self, from: GraphNodeId, to: GraphNodeId) -> f32 {
    self
      .edges_from(from)
      .find(|edge| edge.to == to)
      .map(|edge| edge.weight)
      .unwrap_or(0.0)
  }

  /// Gets the potential neighbours around the given node.
  fn get_neighbours(&self, center: GraphNodeId, results: &mut NeighbourList<GraphNodeId>) {
    for edge in self.edges_from(center) {
      results.push(edge.to);
    }
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

  #[test]
  fn test_basic_graph_pathing() {
    let mut graph = Graph::default();

    let node1 = graph.add_node("a");
    let node2 = graph.add_node("b");
    let node3 = graph.add_node("c");
    let node4 = graph.add_node("d");

    graph.add_edge(node1, node2, 1.0);
    graph.add_edge(node1, node3, 2.0);
    graph.add_edge(node2, node3, 3.0);
    graph.add_edge(node3, node4, 1.0);

    let path = Vec::from(graph.find_path(node1, node4, |_, _| 0.0).unwrap());

    assert_eq!(path, vec![node1, node3, node4]);
  }
}
