//! A simple ECS example for Surreal.

use surreal::prelude::*;

#[derive(Default)]
pub struct Transform {
  pub position: Vector2<f32>,
  pub rotation: f32,
}

#[derive(Default)]
pub struct Sprite {
  pub animation: SpriteAnimation<SpriteState>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SpriteState {
  Idle,
  Walk,
}

impl Default for SpriteState {
  fn default() -> Self {
    SpriteState::Idle
  }
}

fn main() {
  let configuration = Configuration {
    title: "Entity Component Tests",
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let sprites_idle = TextureAtlas::load(&assets, 16, 16, "assets/sprites/spawner-idle.png").unwrap();
    let sprites_walk = TextureAtlas::load(&assets, 16, 16, "assets/sprites/spawner-walk.png").unwrap();

    let mut renderer = RenderContextManager::new(&engine.graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(256., 144., 0., 100.),
      palette: Some(load_built_in_palette(BuiltInPalette::Demichrome4)),
      ..Default::default()
    });

    let mut world = World::default();

    let player = world.spawn();

    player.insert(Transform {
      position: vec2(0., 0.),
      rotation: 0.,
    });

    player.insert(
      SpriteAnimation::default()
        .with_sprites(SpriteState::Idle, sprites_idle)
        .with_sprites(SpriteState::Walk, sprites_walk)
        .with_frames_per_second(8.),
    );

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      let transform = player.get_mut_unchecked::<Transform>();
      let sprite = player.get_mut_unchecked::<SpriteAnimation<SpriteState>>();

      sprite.update(tick.time.delta_time);

      renderer.with(|context: &mut SpriteBatchContext| {
        if let Some(sprite) = sprite.active_sprite() {
          context.batch.draw_sprite(
            &sprite,
            &SpriteOptions {
              position: transform.position,
              rotation: transform.rotation,
              ..Default::default()
            },
          )
        }
      });

      let mut movement = vec2(0., 0.);

      if engine.input.keyboard.is_key_down(Key::W) {
        movement.y -= 1.;
      }

      if engine.input.keyboard.is_key_down(Key::S) {
        movement.y += 1.;
      }

      if engine.input.keyboard.is_key_down(Key::A) {
        movement.x -= 1.;
      }

      if engine.input.keyboard.is_key_down(Key::D) {
        movement.x += 1.;
      }

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }

      if movement.length_squared() > 0. {
        sprite.set_state(SpriteState::Walk);
      } else {
        sprite.set_state(SpriteState::Idle);
      }

      transform.position += movement * 100. * tick.time.delta_time;
      transform.rotation += tick.time.delta_time;
    });
  });
}
