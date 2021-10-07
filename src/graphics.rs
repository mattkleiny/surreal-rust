//! A lightweight and fast cross-platform graphics engine using OpenGL.

pub use buffers::*;
pub use colors::*;
pub use images::*;
pub use meshes::*;
pub use shaders::*;
pub use sprites::*;
pub use textures::*;

mod buffers;
mod colors;
mod images;
mod meshes;
mod shaders;
mod sprites;
mod textures;

/// Represents an error in the graphics subsystem.
pub type GraphicsResult<T> = std::result::Result<T, Error>;

/// Abstracts over a graphics device or GPU.
///
/// Permits interaction with the underlying graphics API through a higher-level abstraction.
pub trait GraphicsDevice {
  fn clear_color_buffer(&mut self, color: Color);
  fn clear_depth_buffer(&mut self);
  fn set_viewport(&mut self, viewport: Viewport);
  fn draw_mesh(&mut self, topology: PrimitiveTopology, vertex_buffer: &Buffer, index_buffer: &Buffer, vertex_count: usize);

  /// Executes all command stored in the given command buffer against the device.
  fn execute_commands(&mut self, buffer: &mut CommandBuffer) {
    while let Some(command) = buffer.dequeue() {
      match command {
        Command::ClearColor(color) => self.clear_color_buffer(color),
        Command::ClearDepth => self.clear_depth_buffer(),
        Command::SetViewport(viewport) => self.set_viewport(viewport),
        Command::DrawMesh { topology, vertex_buffer, index_buffer, vertex_count } => {
          self.draw_mesh(topology, vertex_buffer, index_buffer, vertex_count);
        }
      }
    }
  }
}

/// Commands that can be enqueued in a `CommandBuffer`
pub enum Command<'a> {
  ClearColor(Color),
  ClearDepth,
  SetViewport(Viewport),
  DrawMesh {
    topology: PrimitiveTopology,
    vertex_buffer: &'a Buffer,
    index_buffer: &'a Buffer,
    vertex_count: usize,
  },
}

/// A command buffer that can be used to issue instructions to the GPU.
pub struct CommandBuffer<'a> {
  commands: Vec<Command<'a>>,
}

impl<'a> CommandBuffer<'a> {
  pub fn new() -> Self {
    Self { commands: Vec::new() }
  }

  pub fn enqueue(&mut self, command: Command<'a>) {
    self.commands.push(command);
  }

  pub fn dequeue(&mut self) -> Option<Command> {
    self.commands.pop()
  }
}

/// A viewport for scissoring operations on a `GraphicsDevice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Viewport {
  pub width: usize,
  pub height: usize,
}

/// Represents the different topologies supported for a mesh.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTopology {
  Points,
  Lines,
  Triangles,
  Quads,
}

/// Represents the different blending modes for the graphics pipeline.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BlendingMode {
  None,
}

/// Represents an error with graphics.
#[derive(Debug)]
pub enum Error {
  InvalidBuffer,
  InvalidTexture,
  InvalidShaderProgram,
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Graphics(error)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn command_queue_should_enqueue_and_dequeue() {
    let mut buffer = CommandBuffer::new();

    buffer.enqueue(Command::SetViewport(Viewport { width: 1920, height: 1080 }));
    buffer.enqueue(Command::ClearColor(Color::WHITE));
    buffer.enqueue(Command::ClearDepth);

    while let Some(command) = buffer.dequeue() {
      match command {
        Command::ClearColor(_) => println!("Clearing color buffer"),
        Command::ClearDepth => println!("Clearing depth buffer"),
        Command::SetViewport(viewport) => println!("Setting viewport to {:?}", viewport),
        _ => {}
      }
    }
  }
}