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
  surreal::diagnostics::ConsoleLogger::install(surreal::diagnostics::LevelFilter::Trace);

  let args = Arguments::parse();

  let project = Project::open_or_create(
    &args.project_name.unwrap_or_else(|| "Untitled".to_string()),
    &args.project_path.unwrap_or_else(|| {
      std::env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed parse current directory")
        .to_string()
    }),
  )?;

  let mut host = EditorWindowHost::new();

  host.add_window(MainWindow::new(project))?;
  // host.add_window(ProjectWindow::new(project.clone()));
  // host.add_window(ProjectWindow::new(project.clone()));
  host.run();

  Ok(())
}
