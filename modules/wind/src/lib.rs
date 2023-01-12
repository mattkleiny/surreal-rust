//! An example module for wind simulation in Surreal.
//!
//! This is a culmination of various other crates to build something simple
//! but that tests the ergonomics of the engine.

use std::collections::HashMap;

use surreal::{
  graphics::{RenderContext, Renderer},
  macros::Object,
  maths::Vec3,
  scene::{SceneComponent, SceneContext, SceneNodeId},
  utilities::ServiceProvider,
};

/// Allows an object to emit wind into the game world.
#[derive(Object)]
pub struct WindEmitter {}

impl SceneComponent for WindEmitter {
  fn name(&self) -> &'static str {
    "WindEmitter"
  }

  fn on_render(&mut self, _context: SceneContext, renderer: &mut Renderer) {
    renderer.with(|_context: &mut WindContext| {
      // TODO: render wind
    });
  }
}

/// Receives wind from the wind system, allowing objects to be affected due to wind emitters.
#[derive(Object)]
pub struct WindReceiver {}

impl SceneComponent for WindReceiver {
  fn name(&self) -> &'static str {
    "WindReceiver"
  }

  fn on_disable(&mut self, context: SceneContext) {
    let wind_manager = context.services.get_service_or_default::<WindManager>();

    wind_manager.vorticles.remove(&context.node.id());
  }

  fn on_update(&mut self, context: SceneContext, _delta_time: f32) {
    let wind_manager = context.services.get_service_or_default::<WindManager>();

    wind_manager.vorticles.insert(
      context.node.id(),
      Vorticle {
        _position: context.node.local_position(),
        _velocity: Vec3::ZERO,
      },
    );
  }
}

/// A manager for wind operations.
#[derive(Object, Default)]
struct WindManager {
  vorticles: HashMap<SceneNodeId, Vorticle>,
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
  use surreal::graphics::create_test_graphics;
  use surreal::scene::{SceneEvent, SceneGraph, SceneNodeBuilder};

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

    scene.notify(SceneEvent::Update(0.16));
    scene.notify(SceneEvent::Update(0.16));
    scene.notify(SceneEvent::Update(0.16));
    scene.notify(SceneEvent::Update(0.16));
    scene.notify(SceneEvent::Render(&mut renderer));
  }
}
