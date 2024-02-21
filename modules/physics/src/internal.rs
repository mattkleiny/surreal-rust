use super::*;

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub struct InternalPhysicsBackend {}

#[allow(unused_variables)]
impl PhysicsBackend for InternalPhysicsBackend {
  fn create_world_2d(&self) -> Box<dyn PhysicsWorld2D> {
    todo!()
  }

  fn create_world_3d(&self) -> Box<dyn PhysicsWorld3D> {
    todo!()
  }
}

mod dim2 {
  use common::Vec2;

  struct InternalWorld2D {}

  struct Body {}

  struct Collider {}

  enum ColliderShape {
    Sphere { radius: f32 },
    Box { size: Vec2 },
    Capsule { radius: f32, height: f32 },
    Cylinder { radius: f32, height: f32 },
    Cone { radius: f32, height: f32 },
    ConvexHull { vertices: Vec<Vec2> },
    TriangleMesh { vertices: Vec<Vec2>, indices: Vec<u32> },
    HeightField { size: Vec2, heights: Vec<f32> },
  }

  struct Effector {}

  enum EffectorShape {
    Sphere { radius: f32 },
    Box { size: Vec2 },
    Capsule { radius: f32, height: f32 },
    Cylinder { radius: f32, height: f32 },
  }
}

mod dim3 {
  /// A 3d physics world.
  struct InternalWorld3D {}
}
