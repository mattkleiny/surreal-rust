//! A simple sprite benchmark for Surreal.

use surreal::prelude::*;

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

fn main() {
  let configuration = Configuration {
    title: "Bunnymark",
    size: (WIDTH as u32, HEIGHT as u32),
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let graphics = &engine.graphics;
    let mut interface = UserInterface::new(graphics);

    // set-up assets and rendering
    let sprite = Texture::load(&assets, "assets/sprites/bunny.png").unwrap();
    let region = TextureRegion::from(&sprite);
    let mut renderer = RenderContextManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(WIDTH, HEIGHT, 0., 100.),
      ..Default::default()
    });

    // set-up state
    let mut random = Random::with_thread_local_seed();
    let mut bunnies = Vec::<Bunny>::new();

    engine.run_variable_step(move |engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      // update bunnies
      for bunny in &mut bunnies {
        bunny.update(tick.time.delta_time);
      }

      // draw bunnies
      renderer.begin_frame();
      renderer.with(|context: &mut SpriteBatchContext| {
        for bunny in &bunnies {
          context.batch.draw_sprite(
            &region,
            &SpriteOptions {
              position: bunny.position,
              color: bunny.color,
              ..Default::default()
            },
          );
        }
      });
      renderer.end_frame();

      interface.run(engine, |egui| {
        egui::Window::new("Bunnymark").show(egui, |ui| {
          ui.heading("Statistics");
          ui.label(format!("There are {} bunnies on screen", bunnies.len()));
        });
      });

      // handle input
      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }

      if engine.input.mouse.is_button_down(MouseButton::Left) {
        let position = engine.input.mouse.normalised_position();

        for _ in 0..128 {
          bunnies.push(Bunny {
            position: vec2(position.x * WIDTH - WIDTH / 2., position.y * HEIGHT - HEIGHT / 2.),
            velocity: vec2(random.next::<f32>() * 2. - 1., random.next::<f32>() * 2. - 1.),
            color: Color32::random(),
          });
        }

        info!("There are {:?} bunnies", bunnies.len());
      }

      if engine.input.mouse.is_button_down(MouseButton::Right) {
        for _ in 0..128 {
          bunnies.pop();
        }

        info!("There are {:?} bunnies", bunnies.len());
      }
    });
  });
}

/// Represents a bunny in the benchmark.
struct Bunny {
  position: Vector2<f32>,
  velocity: Vector2<f32>,
  color: Color32,
}

impl Bunny {
  /// Updates the bunny's position.
  pub fn update(&mut self, delta_time: f32) {
    self.position += self.velocity * 100. * delta_time;

    if self.velocity.x < 0. && self.position.x < -WIDTH / 2. {
      self.velocity.x *= -1.;
    }
    if self.velocity.y < 0. && self.position.y < -HEIGHT / 2. {
      self.velocity.y *= -1.;
    }
    if self.velocity.x > 0. && self.position.x > WIDTH / 2. {
      self.velocity.x *= -1.;
    }
    if self.velocity.y > 0. && self.position.y > HEIGHT / 2. {
      self.velocity.y *= -1.;
    }
  }
}
