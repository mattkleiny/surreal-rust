//! An example of using egui in Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Surreal <3 egui",
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let mut renderer = RenderManager::new(&game.host.graphics);

    renderer.configure(EguiContextDescriptor);

    let mut name = "Matt".to_string();
    let mut age = 33;

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(Color::BLACK);

      renderer.with(|context: &mut EguiContext| {
        context.run(|ctx| {
          egui::CentralPanel::default().show(ctx, |ui| {
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
      });

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
