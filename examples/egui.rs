//! An example of using egui in Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = EngineConfig {
    title: "Hello, egui".to_string(),
    size: (1920, 1080),
    update_continuously: false,
    log_level: LevelFilter::Trace,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let mut interface = UserInterface::new(&engine.graphics);

    let mut name = "Matt".to_string();
    let mut age = 33;

    engine.run_variable_step(|engine, _| {
      engine.graphics.clear_color_buffer(Color::rgb(0.1, 0.1, 0.1));

      interface.run(engine, |egui| {
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

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::F7) {
          interface.toggle_profiler();
        }

        if keyboard.is_key_pressed(Key::Escape) {
          engine.quit();
        }
      }
    });
  });
}
