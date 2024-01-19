//! Scene Graph framework for Surreal
//!
//! This module provides a scene graph for Surreal. The scene graph is a
//! hierarchical structure of nodes, each of which can have a transform and
//! a set of components.
//!
//! Components are the things that actually do the work in the scene graph.
//! They are attached to nodes and can be queried by the renderer to determine
//! what to render.

#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

pub use components::*;
pub use graph::*;
pub use tags::*;
pub use transforms::*;

mod components;
mod graph;
mod tags;
mod transforms;

/// A notification for some event that occurred in the scene.
pub enum SceneEvent<'a> {
  Awake,
  Start,
  Enable,
  Disable,
  Destroy,
  Update(f32),
  Render(&'a mut SceneRenderContext<'a>),
}

/// Context for a scene render event.
pub struct SceneRenderContext<'a> {
  renderer: &'a mut graphics::Renderer,
}
