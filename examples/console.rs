//! An example of using the in-game console in Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = EngineConfig {
    title: "In-game console".to_string(),
    size: (1920, 1080),
    transparent_window: true,
    log_level: LevelFilter::Trace,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let mut interface = UserInterface::new(&engine.graphics);
    let mut console_panel = ConsolePanel::new();

    engine.run_variable_step(|engine, _| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      interface.run(engine, |egui| {
        egui::CentralPanel::default().show(egui, |ui| {
          ui.heading("In-game console");
          ui.separator();

          console_panel.show(ui);
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
