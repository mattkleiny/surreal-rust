use std::io;

fn main() -> io::Result<()> {
  #[cfg(windows)] {
    winres::WindowsResource::new()
        // This path can be absolute, or relative to your crate root.
        .set_icon("./isaac.ico")
        .compile()?;
  }

  Ok(())
}