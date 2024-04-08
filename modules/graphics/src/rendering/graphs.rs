use common::{Arena, FastMultiMap};

use super::*;

common::impl_arena_index!(RenderNodeId, "Identifies a node in a render graph.");

/// A graph of render nodes.
///
/// This graph is used to define the order in which render passes should be
/// executed, and is used to automatically schedule the passes in the correct
/// order.
///
/// The graph is a directed acyclic graph, where each node represents a single
/// render pass, and each edge represents a dependency between two passes.
///
/// The graph is built by adding nodes and edges, and then the graph can be
/// executed by iterating over the nodes in topological order.
#[derive(Default)]
pub struct RenderGraph {
  nodes: Arena<RenderNodeId, Box<dyn RenderPass>>,
  edges: FastMultiMap<RenderNodeId, Dependency>,
  topological_order: Vec<RenderNodeId>,
}

/// A dependency of a render node on a resource or another node.
pub enum Dependency {
  Node(RenderNodeId),
  Target(TargetId, ResourceAccessMode),
  Texture(TextureId, ResourceAccessMode),
}

/// The access mode of a resource.
pub enum ResourceAccessMode {
  Read,
  Write,
  ReadWrite,
}

impl RenderGraph {
  /// Returns the number of nodes in the graph.
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  /// Returns `true` if the graph is empty.
  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  /// Adds a new node to the graph.
  pub fn add_node(&mut self, node: impl RenderPass + 'static) -> RenderNodeId {
    let node_id = self.nodes.insert(Box::new(node));
    self.sort_topologically();
    node_id
  }

  /// Removes a node from the graph.
  pub fn remove_node(&mut self, node_id: RenderNodeId) {
    self.nodes.remove(node_id);
    self.sort_topologically();
  }

  /// Adds a dependency to the node.
  pub fn add_dependency(&mut self, node: RenderNodeId, dependency: Dependency) {
    self.edges.insert(node, dependency);
    self.sort_topologically();
  }

  /// Removes all dependency from a node.
  pub fn remove_dependencies(&mut self, node: RenderNodeId) {
    self.edges.remove_all(&node);
    self.sort_topologically();
  }

  /// Clears the graph.
  pub fn clear(&mut self) {
    self.nodes.clear();
    self.edges.clear();
    self.topological_order.clear();
  }

  /// Iterate over the nodes in the graph in topological order.
  pub fn iter(&self) -> impl Iterator<Item = &dyn RenderPass> {
    struct Iter<'a> {
      graph: &'a RenderGraph,
    }

    impl<'a> Iterator for Iter<'a> {
      type Item = &'a dyn RenderPass;

      fn next(&mut self) -> Option<Self::Item> {
        todo!()
      }
    }

    Iter { graph: self }
  }

  /// Mutably iterate over the nodes in the graph in topological order.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut dyn RenderPass> {
    struct IterMut<'a> {
      graph: &'a mut RenderGraph,
    }

    impl<'a> Iterator for IterMut<'a> {
      type Item = &'a mut dyn RenderPass;

      fn next(&mut self) -> Option<Self::Item> {
        todo!()
      }
    }

    IterMut { graph: self }
  }

  /// Sorts the nodes in the graph in topological order and retains the order.
  fn sort_topologically(&mut self) {
    self.topological_order.clear();

    // TODO: implement me
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestPass1 {}
  impl RenderPass for TestPass1 {}

  #[test]
  fn render_graph_should_support_a_good_api_for_building_renderers() {
    let graphics = GraphicsEngine::headless();
    let mut graph = RenderGraph::default();

    let color_target = RenderTarget::new(&graphics, &RenderTargetDescriptor {
      color_attachment: RenderTextureDescriptor {
        width: 1920,
        height: 1080,
        options: TextureOptions::default(),
      },
      depth_attachment: None,
      stencil_attachment: None,
    })
    .unwrap();

    let node1 = graph.add_node(TestPass1 {});
    let node2 = graph.add_node(TestPass1 {});
    let node3 = graph.add_node(TestPass1 {});

    graph.add_dependency(node1, Dependency::Target(color_target.id(), ResourceAccessMode::Write));
    graph.add_dependency(node2, Dependency::Node(node1));
    graph.add_dependency(node3, Dependency::Node(node2));
  }
}
