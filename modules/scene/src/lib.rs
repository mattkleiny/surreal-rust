//! Scene Graph framework for Surreal
//!
//! This module provides a scene graph for Surreal. The scene graph is a
//! hierarchical structure of nodes, each of which can have a transform and
//! a set of components.
//!
//! Components are the things that actually do the work in the scene graph.
//! They are attached to nodes and can be queried by the renderer to determine
//! what to render.

#![feature(impl_trait_in_assoc_type)]

pub use components::*;
pub use graph::*;
pub use transform::*;

mod components;
mod graph;
mod rendering;
mod transform;

use std::borrow::Cow;

use common::collections::FastHashSet;

common::impl_string!(NodePath);
common::impl_string!(Tag);

/// The ID of the layer that a [`SceneNode`] inhabits.
pub type LayerId = u16;

/// A set of one or more [`Tag`]s.
pub type TagSet<'a> = FastHashSet<Tag<'a>>;

/// A notification for some event that occurred in the scene.
pub enum SceneEvent<'a> {
  Awake,
  Start,
  Enable,
  Disable,
  Destroy,
  Update(f32),
  Render(&'a mut graphics::Renderer),
}

impl<'a> NodePath<'a> {
  /// Splits the path into it's first component and the rest of the path.
  #[inline(always)]
  fn split_first(&'a self) -> Option<(&'a str, &'a str)> {
    self.0.split_once('/')
  }
}
