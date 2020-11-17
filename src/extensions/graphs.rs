//! A graph model with support for editor tooling and compilation.

use crate::scripting::{Compilable, Compiler};

type NodeId = usize;

/// A node in the graph.
struct Node<N> {
  id: NodeId,
  kind: NodeKind,
  position: (usize, usize),
  parent: Option<NodeId>,
  children: Vec<NodeId>,
  node: N,
}

impl<N> Node<N> {
  pub fn new(node: N) -> Self {
    Self {
      id: 0,
      kind: NodeKind::Normal,
      position: (0, 0),
      parent: None,
      children: vec![],
      node,
    }
  }
}

/// Describes the different possible types of nodes.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NodeKind {
  Normal,
  Prime,
  Input,
  Output,
}


impl<N> Compilable for Node<N> where N: Compilable {
  type Instruction = N::Instruction;

  fn emit_instructions(&self, compiler: &mut impl Compiler<Instruction=Self::Instruction>) {
    // pass compilation through to the child node
    self.node.emit_instructions(compiler);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  type Node = super::Node<ExampleNode>;

  enum ExampleNode {
    SpawnPrefab
  }

  struct ExampleCompiler {
    instructions: Vec<Instruction>,
  }

  impl ExampleCompiler {
    pub fn new() -> Self {
      Self { instructions: Vec::new() }
    }
  }

  enum Instruction {
    Spawn
  }

  impl Compiler for ExampleCompiler {
    type Instruction = Instruction;

    fn emit_instruction(&mut self, instruction: Self::Instruction) {
      self.instructions.push(instruction);
    }
  }

  impl Compilable for ExampleNode {
    type Instruction = Instruction;

    fn emit_instructions(&self, compiler: &mut impl Compiler<Instruction=Self::Instruction>) {
      compiler.emit_instruction(match self {
        ExampleNode::SpawnPrefab => Instruction::Spawn
      });
    }
  }

  #[test]
  fn it_should_emit_bytecode() {
    let node = Node::new(ExampleNode::SpawnPrefab);
    let mut compiler = ExampleCompiler::new();

    node.emit_instructions(&mut compiler);
  }
}