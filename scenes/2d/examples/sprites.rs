use surreal::assets::Asset;
use surreal::graphics::{Texture, TextureRegion};
use surreal::maths::vec3;
use surreal::scene::{SceneGraph, SceneNodeBuilder};
use surreal_scene2d::SpriteComponent;

mod common;

pub fn main() {
  common::bootstrap("Sprites", |_, assets| {
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
