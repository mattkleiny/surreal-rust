//! A simple sprite benchmark for Surreal.

use surreal::{prelude::*, prototype::*};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

fn main() {
  EngineBuilder::default()
    .with_title("Bunnymark")
    .with_size((WIDTH as u32, HEIGHT as u32))
    .start(|engine, assets| {
      let graphics = &engine.graphics;
      let mut interface = UserInterface::new(graphics)?;

      // set-up assets and rendering
      let sprite = Texture::load(&assets, "assets/sprites/bunny.png")?;
      let region = TextureRegion::from(&sprite);
      let mut renderer = Renderer::new(graphics);

      renderer.add_descriptor(SpriteContextDescriptor {
        projection_view: Mat4::orthographic_rh_gl(
          -WIDTH / 2.,
          WIDTH / 2.,
          HEIGHT / 2.,
          -HEIGHT / 2.,
          0.,
          100.,
        ),
        ..Default::default()
      });

      // set-up state
      let mut random = Random::with_thread_local_seed();
      let mut bunnies = Vec::<Bunny>::new();
      let mut is_updating = true;
      let mut is_rendering = true;

      engine.run_variable_step(move |engine, time| {
        engine
          .graphics
          .clear_color_buffer(Color::rgb(0.1, 0.1, 0.1));

        // update bunnies
        if is_updating {
          for bunny in &mut bunnies {
            bunny.update(time.delta_time);
          }
        }

        // draw bunnies
        if is_rendering {
          renderer.begin_frame();
          renderer.with(|context: &mut SpriteContext| {
            for bunny in &bunnies {
              context.batch.draw_sprite(
                &region,
                &SpriteOptions {
                  position: bunny.position,
                  color: bunny.color,
                  rotation: bunny.rotation.into_radians(),
                  ..Default::default()
                },
              );
            }
          });
          renderer.end_frame();
        }

        interface.run(engine, |egui| {
          egui::Window::new("Bunnymark").show(egui, |ui| {
            ui.heading("Statistics");

            ui.label(format!("There are {} bunnies on screen", bunnies.len()));
            ui.label(format!(
              "Running at {:.2} frames per second",
              1.0 / time.delta_time
            ));

            ui.checkbox(&mut is_updating, "Update bunnies");
            ui.checkbox(&mut is_rendering, "Render bunnies");
          });
        });

        // handle input
        if let Some(keyboard) = &engine.input.keyboard {
          if keyboard.is_key_pressed(Key::F7) {
            interface.toggle_profiler();
          }

          if keyboard.is_key_pressed(Key::Escape) {
            engine.quit();
          }
        }

        if let Some(mouse) = &engine.input.mouse {
          if mouse.is_button_down(MouseButton::Left) {
            let position = mouse.normalised_position();

            for _ in 0..512 {
              bunnies.push(Bunny {
                position: vec2(
                  position.x * WIDTH - WIDTH / 2.,
                  position.y * HEIGHT - HEIGHT / 2.,
                ),
                velocity: vec2(
                  random.next::<f32>() * 2. - 1.,
                  random.next::<f32>() * 2. - 1.,
                ),
                color: Color32::random(),
                rotation: 0.0,
                rotation_speed: f32::random() * 10. - 5.,
              });
            }

            info!("There are {:?} bunnies", bunnies.len());
          }

          if mouse.is_button_down(MouseButton::Right) {
            for _ in 0..512 {
              bunnies.pop();
            }

            info!("There are {:?} bunnies", bunnies.len());
          }
        }
      })
    })
    .expect("An unexpected error occurred");
}

/// Represents a bunny in the benchmark.
struct Bunny {
  position: Vec2,
  velocity: Vec2,
  color: Color32,
  rotation: f32,
  rotation_speed: f32,
}

impl Bunny {
  /// Updates the bunny's position.
  pub fn update(&mut self, delta_time: f32) {
    self.position += self.velocity * 100. * delta_time;
    self.rotation += self.rotation_speed * delta_time;

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
