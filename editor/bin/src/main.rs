//! Entry point for the editor application frontend.

#![windows_subsystem = "windows"]

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
