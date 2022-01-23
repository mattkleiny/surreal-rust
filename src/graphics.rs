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

/// Represents a fallible result in the graphics subsystem.
pub type GraphicsResult<T> = anyhow::Result<T>;

/// An opaque handle to an underlying resource in the `GraphicsServer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GraphicsHandle(u64);

/// A server for the underlying graphics subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level unsafe implementation abstraction, and not
/// imply higher-level constructs and safety, which would instead come from the graphics module
/// primitives.
pub unsafe trait GraphicsServer {
  // commands
  fn execute_command_queue(&mut self, mut commands: CommandBuffer) {
    while let Some(command) = commands.dequeue() {
      match command {
        Command::ClearColor(color) => self.clear_color_buffer(color),
        Command::ClearDepth => self.clear_depth_buffer(),
        Command::SetViewport(viewport) => self.set_viewport(viewport),
        _ => {}
      }
    }
  }

  // intrinsics
  fn set_viewport(&self, viewport: Viewport);
  fn clear_color_buffer(&self, color: Color);
  fn clear_depth_buffer(&self);
  fn flush_commands(&self);

  // buffers
  fn create_buffer(&self) -> GraphicsHandle;
  fn write_buffer_data<T>(&self, handle: GraphicsHandle, data: &[T]);
  fn delete_buffer(&self, buffer: GraphicsHandle);

  // textures
  fn create_texture(&self) -> GraphicsHandle;
  fn write_texture_data<T>(&self, texture: GraphicsHandle, data: &[T]);
  fn delete_texture(&self);

  // shaders
  fn create_shader(&self) -> GraphicsHandle;
  fn delete_shader(&self, shader: GraphicsHandle);
}

/// Commands that can be enqueued in a `CommandBuffer` and replayed at a later date on the graphics
/// server or graphics pipeline.
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