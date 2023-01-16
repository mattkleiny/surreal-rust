//! An example module for wind simulation in Surreal.
//!
//! This is a culmination of various other crates to build something simple
//! but that tests the ergonomics of the engine.

use surreal::{
  collections::FastHashMap,
  graphics::{RenderContext, Renderer},
  macros::Object,
  maths::Vec3,
  scene::{SceneComponent, SceneContext, SceneNodeId},
  utilities::ServiceProvider,
};

/// Emits wind into the game world, allowing [`WindReceiver`]s to react to forces.
#[derive(Object)]
pub struct WindEmitter {}

impl SceneComponent for WindEmitter {
  fn on_render(&mut self, _context: SceneContext, renderer: &mut Renderer) {
    renderer.with(|_context: &mut WindContext| {
      // TODO: render wind via the wind context (compute shader)
    });
  }
}

/// Receives wind from the wind system, allowing objects to be affected by [`WindEmitter`]s.
#[derive(Object)]
pub struct WindReceiver {}

impl SceneComponent for WindReceiver {
  fn on_disable(&mut self, context: SceneContext) {
    let manager = context.services.get_service_or_default::<WindManager>();

    manager.vorticles.remove(&context.node.id());
  }

  fn on_update(&mut self, context: SceneContext, _delta_time: f32) {
    let manager = context.services.get_service_or_default::<WindManager>();

    manager.vorticles.insert(
      context.node.id(),
      Vorticle {
        _position: context.node.local_position(),
        _velocity: Vec3::ZERO,
      },
    );

    println!("Vorticle count: {}", manager.vorticles.len());
  }
}

/// A manager for wind operations.
#[derive(Object, Default)]
struct WindManager {
  vorticles: FastHashMap<SceneNodeId, Vorticle>,
}

struct Vorticle {
  _position: Vec3,
  _velocity: Vec3,
}

/// A [`RenderContext`] for wind rendering.
#[derive(Object)]
struct WindContext {}

impl RenderContext for WindContext {}

#[cfg(test)]
mod tests {
  use surreal::{
    graphics::create_test_graphics,
    scene::{SceneGraph, SceneNodeBuilder},
  };

  use super::*;

  #[test]
  fn this_should_work() {
    let mut scene = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_component(WindEmitter {})
        .with_component(WindReceiver {})
        .build(),
    );

    let mut renderer = Renderer::new(&create_test_graphics());

    scene.update(0.16);
    scene.update(0.16);
    scene.update(0.16);
    scene.update(0.16);
    scene.update(0.16);
    scene.render(&mut renderer);
  }
}
