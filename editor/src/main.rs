use clap::Parser;
use surreal_editor::*;

/// Commands-line arguments for the Surreal editor.
#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
  #[arg(long)]
  project_name: Option<String>,
  #[arg(long)]
  project_path: Option<String>,
}

/// Entry point for the Surreal editor application.
fn main() -> surreal::Result<()> {
  surreal::diagnostics::ConsoleLoggerBuilder::new()
    .with_level(surreal::diagnostics::LevelFilter::Trace)
    .install();

  let args = Arguments::parse();

  let mut host = EditorWindowHost::new();
  let project = Project::open_or_create(
    args.project_name.unwrap_or_else(|| "Untitled".to_string()),
    args.project_path.unwrap_or_else(|| {
      std::env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed parse current directory")
        .to_string()
    }),
  )?;

  host.add_window(ProjectWindow::new(project));

  Ok(host.run())
}
