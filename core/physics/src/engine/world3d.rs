use super::*;

/// A 3D physics world.
#[derive(Default)]
pub struct PhysicsWorld3D {}

#[allow(unused_variables)]
impl PhysicsWorld for PhysicsWorld3D {
  type Vector = Vec3;

  fn tick(&self, _delta: f32) {
    todo!()
  }

  fn collider_create(&self) -> Result<ColliderId, ColliderError> {
    todo!()
  }

  fn collider_get_position(&self, id: ColliderId) -> Result<Self::Vector, ColliderError> {
    todo!()
  }

  fn collider_set_position(&self, id: ColliderId, position: Self::Vector) -> Result<(), ColliderError> {
    todo!()
  }

  fn collider_delete(&self, id: ColliderId) -> Result<(), ColliderError> {
    todo!()
  }

  fn body_create(&self) -> Result<BodyId, BodyError> {
    todo!()
  }

  fn body_get_position(&self, id: BodyId) -> Result<Self::Vector, BodyError> {
    todo!()
  }

  fn body_set_position(&self, id: BodyId, position: Self::Vector) -> Result<(), BodyError> {
    todo!()
  }

  fn body_get_velocity(&self, id: BodyId) -> Result<Self::Vector, BodyError> {
    todo!()
  }

  fn body_set_velocity(&self, id: BodyId, velocity: Self::Vector) -> Result<(), BodyError> {
    todo!()
  }

  fn body_delete(&self, id: BodyId) -> Result<(), BodyError> {
    todo!()
  }
}
