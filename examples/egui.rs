//! An example of using egui in Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Hello, egui",
    size: (1920, 1080),
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine| {
    let mut canvas = UserInterface::new(&engine.graphics);

    let mut name = "Matt".to_string();
    let mut age = 33;

    engine.run_variable_step(|engine, tick| {
      engine
        .graphics
        .clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      canvas.run(&engine.input, |egui| {
        egui::Window::new("Surreal ❤ egui").show(egui, |ui| {
          ui.heading("My egui Application");

          ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut name);
          });

          ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));

          if ui.button("Click each year").clicked() {
            age += 1;
          }

          ui.label(format!("Hello '{}', age {}", name, age));
        });
      });

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
