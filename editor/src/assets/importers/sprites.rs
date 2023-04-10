use std::{ops::Deref, sync::Arc};

use surreal::{
  graphics::{
    Color32, Material, RenderContext, Renderer, SpriteBatch, SpriteOptions, Texture, TextureRegion,
  },
  io::VirtualPath,
  macros::Object,
  maths::{vec2, Guid, Rectangle, Vec2},
  scene::{SceneComponent, SceneContext},
};

use crate::{Asset, AssetImporter};

/// An importer for raw sprite assets.
#[derive(Default)]
pub struct SpriteImporter {}

impl AssetImporter for SpriteImporter {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    path.extension() == "png"
      || path.extension() == "jpg"
      || path.extension() == "jpeg"
      || path.extension() == "bmp"
  }

  fn import(&self, path: &VirtualPath) -> surreal::Result<Box<dyn Asset>> {
    todo!()
  }
}

/// An asset importer for Aseprite files.
///
/// This importer will load the Aseprite file and convert it into a
/// [`SpriteSheet`] and a set of [`SpriteAnimation`]s.
#[derive(Default)]
pub struct AsepriteImporter {
  pub pixels_per_unit: f32,
  pub transparent_mask: Option<Color32>,
}

impl AssetImporter for AsepriteImporter {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    path.extension() == "ase" || path.extension() == "aseprite"
  }

  fn import(&self, path: &VirtualPath) -> surreal::Result<Box<dyn Asset>> {
    todo!()
  }
}

// TODO: implement resource database?

pub struct Ref<R> {
  _resource_id: Guid,
  _instance: Option<Arc<R>>,
}

impl<R> Ref<R> {
  pub fn from_id(resource_id: Guid) -> Self {
    Self {
      _resource_id: resource_id,
      _instance: None,
    }
  }

  pub fn from_instance(instance: Arc<R>) -> Self {
    Self {
      _resource_id: Guid::default(),
      _instance: Some(instance),
    }
  }
}

impl<R> From<R> for Ref<R> {
  fn from(instance: R) -> Self {
    Self::from_instance(Arc::new(instance))
  }
}

impl<R> From<Arc<R>> for Ref<R> {
  fn from(instance: Arc<R>) -> Self {
    Self::from_instance(instance)
  }
}

impl<R> Default for Ref<R> {
  fn default() -> Self {
    Self {
      _resource_id: Guid::default(),
      _instance: None,
    }
  }
}

impl<R> Deref for Ref<R> {
  type Target = R;

  fn deref(&self) -> &Self::Target {
    todo!()
  }
}

#[derive(Default)]
pub struct Sprite {
  pub name: String,
  pub offset: Vec2,
  pub pivot: Vec2,
  pub bounds: Rectangle,
  pub texture: Ref<Texture>,
}

#[derive(Default)]
pub struct SpriteSheet {
  pub texture: Ref<Texture>,
  pub sprites: Vec<Ref<Sprite>>,
}

#[derive(Default)]
pub struct SpriteFrame {
  pub sprite: Ref<Sprite>,
  pub duration: f32,
}

#[derive(Default)]
pub struct SpriteAnimation {
  pub name: String,
  pub frames: Vec<SpriteFrame>,
  pub is_looping: bool,
}

#[derive(Object)]
pub struct SpriteRenderContext {
  pub sprite_batch: SpriteBatch,
  pub material: Material,
}

impl RenderContext for SpriteRenderContext {
  fn on_begin_frame(&mut self) {
    self.sprite_batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.sprite_batch.flush();
  }
}

#[derive(Default, Object)]
pub struct SpriteAnimator {
  pub animation: Ref<SpriteAnimation>,
  pub current_frame: usize,
  pub tint: Color32,
  pub frame_time: f32,
  pub play_on_awake: bool,
  pub is_playing: bool,
}

impl SceneComponent for SpriteAnimator {
  fn on_start(&mut self, context: SceneContext) {
    if self.play_on_awake {
      self.is_playing = true;
    }
  }

  fn on_update(&mut self, context: SceneContext, delta_time: f32) {
    if !self.is_playing {
      return; // nothing to do
    }

    if let Some(frame) = &self.animation.frames.get(self.current_frame) {
      // update frame time
      self.frame_time += delta_time;

      // handle frame change
      if self.frame_time >= frame.duration {
        self.frame_time = 0.0;
        self.current_frame += 1;

        // handle end of animation
        if self.current_frame >= self.animation.frames.len() {
          if self.animation.is_looping {
            self.current_frame = 0;
          } else {
            self.is_playing = false;
          }
        }
      }
    }
  }

  fn on_draw(&mut self, context: SceneContext, renderer: &mut Renderer) {
    if let Some(frame) = &self.animation.frames.get(self.current_frame) {
      let position = context.node.global_position();

      renderer.with(|context: &mut SpriteRenderContext| {
        let region = TextureRegion {
          texture: frame.sprite.texture.clone(),
          offset: frame.sprite.offset.as_uvec2(),
          size: frame.sprite.bounds.size().as_uvec2(),
        };

        context.sprite_batch.draw_sprite(
          &region,
          &SpriteOptions {
            position: vec2(position.x, position.y),
            color: self.tint,
            ..SpriteOptions::default()
          },
        );
      });
    }
  }
}
