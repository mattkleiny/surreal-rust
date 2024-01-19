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
pub use transform::*;

mod components;
mod graph;
mod transform;

use std::borrow::Cow;

use common::collections::FastHashSet;

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

/// The ID of the layer that a [`SceneNode`] inhabits.
pub type LayerId = u16;

/// A set of one or more [`Tag`]s.
pub type TagSet<'a> = FastHashSet<Tag<'a>>;

/// A tag that can be applied to a [`SceneNode`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag<'a>(Cow<'a, str>);

impl<'a> From<&'a str> for Tag<'a> {
  fn from(value: &'a str) -> Self {
    Self(Cow::Borrowed(value))
  }
}

impl<'a> From<String> for Tag<'a> {
  fn from(value: String) -> Self {
    Self(Cow::Owned(value))
  }
}

/// Encapsulates a path to a [`SceneNode`] in a [`SceneGraph`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodePath<'a>(&'a str);

impl<'a> NodePath<'a> {
  /// Splits the path into it's first component and the rest of the path as two
  /// pieces.
  pub fn split_first(&self) -> Option<(&'a str, &'a str)> {
    self.0.split_once('/')
  }
}

impl<'a> From<&'a str> for NodePath<'a> {
  fn from(value: &'a str) -> Self {
    Self(value)
  }
}
