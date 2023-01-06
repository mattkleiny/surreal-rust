//! Graph management tools

use slotmap::{SecondaryMap, SlotMap};

slotmap::new_key_type! {
  /// The ID for a [`GraphNode`].
  pub struct NodeId;

  /// The ID for a [`GraphInput`].
  struct InputId;

  /// The ID for a [`GraphOutput`].
  struct OutputId;
}

/// A graph of [`GraphNode`]s.
#[derive(Serialize, Deserialize)]
pub struct Graph<D = ()> {
  /// The [`GraphNode`]s themselves.
  nodes: SlotMap<NodeId, GraphNode<D>>,
  inputs: SlotMap<InputId, Input<D>>,
  outputs: SlotMap<OutputId, Output<D>>,
  connections: SecondaryMap<InputId, OutputId>,
}

/// A single node in a [`Graph`].
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphNode<D = ()> {
  id: NodeId,
  label: Option<String>,
  inputs: Vec<(String, InputId)>,
  outputs: Vec<(String, OutputId)>,
  data: D,
}

/// An input into a [`Graph`].
#[derive(Serialize, Deserialize, Debug)]
struct Input<D> {
  id: InputId,
  value: Option<D>,
}

/// An output from a [`Graph`].
#[derive(Serialize, Deserialize, Debug)]
struct Output<D> {
  id: OutputId,
  value: Option<D>,
}

impl<D> Default for Graph<D> {
  fn default() -> Self {
    Self {
      nodes: SlotMap::with_key(),
      inputs: SlotMap::with_key(),
      outputs: SlotMap::with_key(),
      connections: SecondaryMap::new(),
    }
  }
}

impl<D> Graph<D> {
  /// Adds a new [`GraphNode`] to the graph.
  pub fn add_node(&mut self, label: String, user_data: D) {
    self.nodes.insert_with_key(|node_id| GraphNode {
      id: node_id,
      label: Some(label),
      inputs: Vec::default(),
      outputs: Vec::default(),
      data: user_data,
    });
  }

  /// Iterates the [`GraphNode`]s of this graph.
  pub fn iter(&self) -> impl Iterator<Item = &GraphNode<D>> {
    self.nodes.iter().map(|(_, node)| node)
  }

  /// Mutably iterates the [`GraphNode`]s of this graph.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut GraphNode<D>> {
    self.nodes.iter_mut().map(|(_, node)| node)
  }
}

impl<'a, D> IntoIterator for &'a Graph<D> {
  type Item = &'a GraphNode<D>;
  type IntoIter = impl Iterator<Item = &'a GraphNode<D>>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, D> IntoIterator for &'a mut Graph<D> {
  type Item = &'a mut GraphNode<D>;
  type IntoIter = impl Iterator<Item = &'a mut GraphNode<D>>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use crate::io::Serializable;

  use super::*;

  #[test]
  fn graph_should_build_simple_connections() {
    let mut graph = Graph::<String>::default();

    graph.add_node("Node 1".to_string(), "Value 1".to_string());
    graph.add_node("Node 2".to_string(), "Value 2".to_string());

    for node in &graph {
      println!("Node: {:#?}", node);
    }

    println!("{}", graph.to_json().unwrap());
  }
}
