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
pub use rendering::*;
pub use transform::*;

mod components;
mod graph;
mod rendering;
mod transform;

use std::borrow::Cow;

use common::FastHashSet;

common::impl_cow_string!(NodePath);
common::impl_cow_string!(Tag);

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
  TransformChanged,
}

#[cfg(test)]
mod tests {
  use common::{Angle, Vec2};

  use super::*;

  #[test]
  fn test_basic_scene_api() {
    let graphics = graphics::GraphicsEngine::headless();

    let mut renderer = graphics::Renderer::new(&graphics);
    let mut scene = SceneGraph::new(
      SceneNodeBuilder::<Transform2D>::default()
        .with_name("root")
        .with_position(Vec2::new(0.0, 0.0))
        .with_child(
          SceneNodeBuilder::<Transform2D>::default()
            .with_name("child 1")
            .with_position(Vec2::new(1.0, 1.0)),
        )
        .with_child(
          SceneNodeBuilder::<Transform2D>::default()
            .with_name("child 2")
            .with_rotation(Angle::Degrees(30.0)),
        )
        .build(),
    );

    scene.update(0.16);
    scene.render(&mut renderer);

    println!("{:#?}", scene);
  }
}
