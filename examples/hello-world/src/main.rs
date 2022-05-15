use surreal::prelude::*;

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  let mut context = ImmediateModeContext::new();

  platform.run(|platform| unsafe {
    platform.graphics_server.clear_color_buffer(Color::BLACK);

    context.run(|context| {
      egui::Window::new("Hello, Surreal")
          .resizable(true)
          .show(context, |ui| {
            if ui.button("Click me").clicked() {
              println!("It works!");
            }
          });
    });

    context.flush();
  });
}
