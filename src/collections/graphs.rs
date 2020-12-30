use crate::collections::{Arena, ArenaIndex};

pub type NodeId = ArenaIndex;

// A graph of nodes with arbitrary data.
struct Graph<V> {
  nodes: Arena<Node<V>>,
  edges: Vec<Edge>,
}

impl<V> Graph<V> {
  pub fn new() -> Self {
    Self {
      nodes: Arena::new(),
      edges: Vec::new(),
    }
  }

  /// Creates a new root node with the given value.
  pub fn create_root_node(&mut self, value: V) -> NodeId {
    self.nodes.insert(Node { value })
  }

  /// Creates a new node with the given value as a child of the given other node.
  pub fn create_child_node(&mut self, parent: NodeId, value: V) -> Option<NodeId> {
    if let Some(root) = self.nodes.get(parent) {
      let child = self.nodes.insert(Node { value });
      self.edges.push(Edge { from: parent, to: child });

      Some(child)
    } else {
      None
    }
  }

  /// Creates an edge from the given node to the given node.
  pub fn create_edge(&mut self, from: NodeId, to: NodeId) {
    unimplemented!()
  }

  /// Clears the entire graph.
  pub fn clear(&mut self) {
    self.nodes.clear();
    self.edges.clear();
  }
}

/// Represents a single node in a graph.
struct Node<V> {
  value: V,
}

/// Represents an edge between two nodes in a graph.
struct Edge {
  from: NodeId,
  to: NodeId,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug)]
  enum Node {
    SpawnPrefab,
    DestroyObject,
  }

  #[test]
  fn graph_should_build_a_simple_graph() {
    let mut graph = Graph::new();

    let root = graph.create_root_node(Node::SpawnPrefab);
    let child1 = graph.create_child_node(root, Node::DestroyObject).unwrap();
    let child2 = graph.create_child_node(root, Node::DestroyObject).unwrap();
  }
}