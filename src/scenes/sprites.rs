//! Sprite components for use in scene rendering.

use std::collections::HashMap;

use crate::{
  assets::Handle,
  graphics::{SpriteBatch, SpriteOptions, Texture, TextureRegion},
  maths::Vector2,
};

/// Represents a sprite in the scene.
#[derive(Default)]
pub struct Sprite {
  animations: HashMap<String, SpriteAnimation>,
  current_animation: String,
}

impl Sprite {
  /// Returns the current animation in use on the sprite.
  pub fn current_animation(&self) -> &String {
    &self.current_animation
  }

  /// Sets the current animation in use for the sprite.
  pub fn set_animation(&mut self, animation: &str) {
    self.current_animation = animation.to_string();
  }

  /// Draws the sprite to the given sprite batch.
  pub fn draw(&mut self, position: Vector2<f32>, batch: &mut SpriteBatch, delta_time: f32) {
    if let Some(animation) = self.animations.get_mut(&self.current_animation) {
      animation.frame_time += delta_time;

      if animation.frame_time >= animation.frame_duration {
        animation.frame_index = (animation.frame_index + 1) % animation.frames.len();
      }

      if let Some(frame) = animation.frames.get(animation.frame_index) {
        batch.draw_sprite(
          &TextureRegion {
            texture: frame.texture.as_ref(),
            offset: frame.offset,
            size: frame.size,
          },
          &SpriteOptions {
            position,
            ..Default::default()
          },
        );
      }
    }
  }
}

#[derive(Default)]
pub struct SpriteAnimation {
  pub frames: Vec<SpriteFrame>,
  pub frame_duration: f32,
  frame_index: usize,
  frame_time: f32,
}

pub struct SpriteFrame {
  pub texture: Handle<Texture>,
  pub offset: Vector2<u32>,
  pub size: Vector2<u32>,
}

pub struct SpriteSheet {
  texture: Handle<Texture>,
  entries: HashMap<String, SpriteSheetEntry>,
}

pub struct SpriteSheetEntry {
  name: String,
  offset: Vector2<u32>,
  size: Vector2<u32>,
}

#[cfg(test)]
mod tests {
  use crate::scenes::Scene;

  use super::*;

  #[test]
  fn sprite_should_be_insertable_into_scene() {
    let mut scene = Scene::default();

    scene.spawn().insert(Sprite::default());
  }
}
