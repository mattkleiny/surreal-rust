//! AI components for games.
//!
//! Decision making, pathfinding, automata and tooling.

/// An 'automata' is a thinking object that can create decisions.
pub trait Automata<M = ()> {
  fn think(&mut self, memory: &mut M, delta_time: f32);
}
