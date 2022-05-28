//! Diagnostic utilities

/// Displays the profiler window.
pub fn display_profiler_window(context: &egui::Context) -> bool {
  puffin_egui::profiler_window(context)
}