use surreal_editor::*;

/// Entry point for the Surreal editor application.
fn main() -> surreal::Result<()> {
  surreal::diagnostics::ConsoleLoggerBuilder::new()
    .with_level(surreal::diagnostics::LevelFilter::Trace)
    .install();

  let mut host = EditorWindowHost::new();

  host.add_window(ProjectWindow::new(Project::open_or_create(
    "Test Project",
    std::env::current_dir()?
      .to_str()
      .ok_or(surreal::anyhow!("Failed to determine current directory"))?,
  )?));

  Ok(host.run())
}
