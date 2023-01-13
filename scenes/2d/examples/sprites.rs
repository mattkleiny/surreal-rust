use surreal::{
  assets::Asset,
  graphics::{Texture, TextureRegion},
  maths::vec3,
  scene::{SceneGraph, SceneNodeBuilder},
};
use surreal_scene2d::SpriteComponent;

mod common;

pub fn main() {
  common::run_example("Sprites", |_, assets| {
    let texture = Texture::load(assets, "local://assets/sprites/bunny.png").unwrap();
    let region = TextureRegion::from(&texture);

    SceneGraph::new(
      SceneNodeBuilder::default()
        .with_name("Sprite")
        .with_local_position(vec3(0., 0., 0.))
        .with_component(SpriteComponent { region }),
    )
  });
}
