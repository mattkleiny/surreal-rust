use std::io;

fn main() -> io::Result<()> {
  // purely to create icons for example projects
  #[cfg(windows)] {
    winres::WindowsResource::new()
      .set_icon("./surreal.ico")
      .compile()?;
  }

  Ok(())
}