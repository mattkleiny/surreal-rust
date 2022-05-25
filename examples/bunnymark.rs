//! A simple sprite benchmark for Surreal.

#![windows_subsystem = "windows"]

use winit::event::VirtualKeyCode::W;
use surreal::prelude::*;

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

/// Represents a bunny in the benchmark.
struct Bunny {
  position: Vector2<f32>,
  velocity: Vector2<f32>,
}

impl Bunny {
  /// Creates a new bunny.
  pub fn new() -> Self {
    Self {
      position: Vector2::new(0., 0.),
      velocity: Vector2::new(0., 0.),
    }
  }

  /// Updates the bunny's position.
  pub fn update(&mut self, delta_time: f32) {
    self.position += self.velocity * 100. * delta_time;

    if self.velocity.x < 0. && self.position.x < -WIDTH / 2. { self.position.x = WIDTH / 2. }
    if self.velocity.y < 0. && self.position.y < -HEIGHT / 2. { self.position.y = HEIGHT / 2. }
    if self.velocity.x > 0. && self.position.x > WIDTH / 2. { self.position.x = -WIDTH / 2. }
    if self.velocity.y > 0. && self.position.y > HEIGHT / 2. { self.position.y = -HEIGHT / 2. }
  }
}

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Bunnymark",
    size: (WIDTH as u32, HEIGHT as u32),
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    // set-up rendering
    let shader = load_standard_shader(&game.host.graphics);
    let mut material = Material::new(&game.host.graphics, &shader);
    let mut batch = SpriteBatch::new(&game.host.graphics);
    let mut texture = Texture::new(&game.host.graphics);
    let image = Image::from_path("assets/sprites/bunny.png", None).expect("Failed to load sprite image");

    texture.write_image(&image);

    // TODO: upside down bunnies?
    let projection_view = Matrix4x4::create_orthographic(WIDTH, HEIGHT, 0., 100.);

    material.set_uniform("u_projectionView", &projection_view);
    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    let mut random = Random::new();
    let mut bunnies = Vec::<Bunny>::new();

    // TODO: simplify this
    let region = TextureRegion::from(&texture);

    game.run_variable_step(move |context| {
      context.host.graphics.clear_color_buffer(Color::BLACK);

      // update bunnies
      for bunny in &mut bunnies {
        bunny.update(context.time.delta_time);
      }

      // draw bunnies
      batch.begin(&material);

      for bunny in &bunnies {
        batch.draw(&region, SpriteOptions {
          position: bunny.position,
          ..Default::default()
        });
      }

      batch.flush();

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_down(Key::Space) {
          // add in groups of 64
          for _ in 0..128 {
            bunnies.push(Bunny {
              position: vec2(0., 0.),
              velocity: vec2(
                random.next::<f32>() * 2. - 1.,
                random.next::<f32>() * 2. - 1.,
              ),
            });
          }
          println!("There are {:?} bunnies", bunnies.len());
        }
      }
    });
  });
}
