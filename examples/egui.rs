//! An example of using egui in Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Surreal <3 egui",
    size: (1920, 1080),
    ..Default::default()
  });

  Game::start(platform, |mut game, _| {
    let mut renderer = RenderManager::new(&game.host.graphics);

    renderer.configure(UserInterfaceContextDescriptor {
      projection_view: Matrix4x4::create_orthographic(1920., 1080., 0., 100.),
    });

    let mut name = "Matt".to_string();
    let mut age = 33;

    game.run_variable_step(|game| {
      game.host.graphics.clear_color_buffer(Color::BLACK);

      renderer.with(|context: &mut UserInterfaceContext| {
        context.run(&game.host.input, |egui| {
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
          });
        });
      });

      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }
    });
  });
}
