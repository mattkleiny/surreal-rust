//! The user interface for the Surreal editor

#[derive(Default)]
pub struct Editor {}

impl Editor {
  pub fn draw(&mut self, egui: &egui::Context) {
    egui::Window::new("Surreal Editor").show(egui, |ui| {
      if ui.button("Test").clicked() {
        println!("Test");
      }
    });
  }
}
