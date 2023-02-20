//! An example of using the in-game console in Surreal.

use surreal::prelude::*;

fn main() {
  EngineBuilder::default()
    .with_title("In-game console")
    .with_size((1920, 1080))
    .with_log_level(LevelFilter::Trace)
    .start(|engine, _| {
      let mut interface = UserInterface::new(&engine.graphics)?;
      let mut console_panel = ConsolePanel::new();

      engine.run_variable_step(|engine, _| {
        engine
          .graphics
          .clear_color_buffer(Color::rgb(0.1, 0.1, 0.1));

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
      })
    })
    .expect("An unexpected error occurred");
}
