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
pub use nodes::*;
pub use tags::*;

mod components;
mod graph;
mod nodes;
mod tags;

/// A notification for some event that occurred in the scene.
pub enum SceneEvent<'a> {
  Awake,
  Start,
  Enable,
  Disable,
  Destroy,
  Update(f32),
  Draw(&'a mut SceneRenderContext<'a>),
}

/// Context for a [`SceneEvent`].
#[derive(Default)]
pub struct SceneContext<'a> {
  _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Default)]
pub struct SceneRenderContext<'a> {
  _phantom: std::marker::PhantomData<&'a ()>,
}
