//! Basic entity management facilities.

use std::ops::{Deref, DerefMut};

use crate::{
  collections::Arena,
  graphics::*,
  maths::{vec2, Vector2},
  prelude::ArenaIndex,
};

/// A simple scene that can be used for prototyping purposes.
#[derive(Default)]
pub struct Scene {
  pub camera: Camera,
  pub entities: EntityManager,
}

impl RenderScene for Scene {
  fn cull_visible_objects(&self, _frustum: &CameraFrustum, _results: &mut Vec<CullingResult>) {
    todo!()
  }

  fn render(&self, _id: u64, _manager: &mut RenderContextManager) {
    todo!()
  }
}

/// A simple camera that can be used in a `Scene`.
pub struct Camera {
  pub position: Vector2<f32>,
  pub rotation: f32,
  pub zoom: f32,
  pub width: f32,
  pub height: f32,
  pub near: f32,
  pub far: f32,
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      position: vec2(0., 0.),
      rotation: 0.,
      zoom: 0.,
      width: 256.,
      height: 144.,
      near: 0.,
      far: 100.,
    }
  }
}

impl RenderCamera for Camera {
  fn compute_frustum(&self) -> CameraFrustum {
    todo!()
  }
}

/// A manager for entities in a scene/game.
#[derive(Default)]
pub struct EntityManager {
  entities: Arena<Entity>,
}

/// An entity in an entity manager.
pub struct Entity {}

/// A reference to an entity in an `EntityManager`.
pub struct EntityRef {
  pub id: ArenaIndex,
  manager: *mut EntityManager,
}

impl Deref for EntityRef {
  type Target = Entity;

  fn deref(&self) -> &Self::Target {
    let manager = unsafe { &*self.manager };

    manager.entities.get(self.id).unwrap()
  }
}

impl DerefMut for EntityRef {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let manager = unsafe { &mut *self.manager };

    manager.entities.get_mut(self.id).unwrap()
  }
}
