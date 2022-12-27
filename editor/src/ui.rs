//! The user interface for the Surreal editor

pub fn editor(egui: &egui::Context) {
  egui::Window::new("Surreal Editor").show(egui, |ui| {
    if ui.button("Test").clicked() {
      println!("Test");
    }
  });
}
