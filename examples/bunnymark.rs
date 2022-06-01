//! A simple sprite benchmark for Surreal.

use surreal::prelude::*;

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Bunnymark",
    size: (WIDTH as u32, HEIGHT as u32),
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let graphics = &game.host.graphics;

    // set-up assets and rendering
    let sprite: &Texture = assets.load_asset("assets/sprites/bunny.png").expect("Failed to load sprite image");
    let region = TextureRegion::from(sprite);

    let mut renderer = RenderManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::create_orthographic(WIDTH, HEIGHT, 0., 100.),
      ..Default::default()
    });

    // set-up state
    let mut random = Random::new();
    let mut bunnies = Vec::<Bunny>::new();

    game.run_variable_step(move |game| {
      game.host.graphics.clear_color_buffer(Color::BLACK);

      // update bunnies
      for bunny in &mut bunnies {
        bunny.update(game.time.delta_time);
      }

      // draw bunnies
      renderer.with(|pass: &mut SpriteBatchContext| {
        for bunny in &bunnies {
          pass.batch.draw(&region, SpriteOptions {
            position: bunny.position,
            ..Default::default()
          });
        }
      });
      
      // handle input
      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }

      if let Some(mouse) = game.host.input.mouse_device() {
        if mouse.is_button_down(MouseButton::Left) {
          let position = mouse.normalised_position();

          for _ in 0..128 {
            bunnies.push(Bunny {
              position: vec2(
                position.x * WIDTH - WIDTH / 2.,
                position.y * HEIGHT - HEIGHT / 2.,
              ),
              velocity: vec2(
                random.next::<f32>() * 2. - 1.,
                random.next::<f32>() * 2. - 1.,
              ),
            });
          }

          println!("There are {:?} bunnies", bunnies.len());
        }

        if mouse.is_button_down(MouseButton::Right) {
          for _ in 0..128 {
            bunnies.pop();
          }
        }
      }
    });
  });
}

/// Represents a bunny in the benchmark.
struct Bunny {
  position: Vector2<f32>,
  velocity: Vector2<f32>,
}

impl Bunny {
  /// Updates the bunny's position.
  pub fn update(&mut self, delta_time: f32) {
    self.position += self.velocity * 100. * delta_time;

    if self.velocity.x < 0. && self.position.x < -WIDTH / 2. { self.position.x = WIDTH / 2. }
    if self.velocity.y < 0. && self.position.y < -HEIGHT / 2. { self.position.y = HEIGHT / 2. }
    if self.velocity.x > 0. && self.position.x > WIDTH / 2. { self.position.x = -WIDTH / 2. }
    if self.velocity.y > 0. && self.position.y > HEIGHT / 2. { self.position.y = -HEIGHT / 2. }
  }
}
