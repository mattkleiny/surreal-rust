use surreal::maths::{vec3, Quat};
use surreal::scene::{SceneGraph, SceneNodeBuilder};

mod common;

pub fn main() {
  common::bootstrap("Sprites", |engine, _| {
    SceneGraph::new(
      SceneNodeBuilder::default()
        .with_name("Sprite")
        .with_local_position(vec3(0., 0., 0.))
        .with_local_rotation(Quat::from_rotation_z(0.25)),
    )
  });
}
