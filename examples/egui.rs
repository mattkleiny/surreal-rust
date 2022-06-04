//! An example of using egui in Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Surreal <3 egui",
    size: (1920, 1080),
    ..Default::default()
  };

  Engine::start(configuration, |mut engine| {
    let mut canvas = UserInterfaceCanvas::new(&engine.graphics);

    // TODO: make this easier to use
    engine.input.pixels_per_point = 1.2;
    canvas.set_pixels_per_point(1.2);

    let mut name = "Matt".to_string();
    let mut age = 33;

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::BLACK);

      canvas.run(&engine.input, |egui| {
        egui::CentralPanel::default().show(egui, |ui| {
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

          use egui::plot::{Line, Plot, Value, Values};

          let sin = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            Value::new(x, x.sin())
          });
          let line = Line::new(Values::from_values_iter(sin));

          Plot::new("my_plot")
            .view_aspect(2.0)
            .show(ui, |plot_ui| plot_ui.line(line));
        });
      });

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
