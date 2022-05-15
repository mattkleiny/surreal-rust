//! A lightweight cross-platform graphics engine.

use std::ops::Deref;
use std::rc::Rc;
pub use buffers::*;
pub use colors::*;
pub use materials::*;
pub use meshes::*;
pub use shaders::*;

mod buffers;
mod colors;
mod materials;
mod meshes;
mod shaders;

/// Represents a fallible result in the graphics subsystem.
pub type GraphicsResult<T> = anyhow::Result<T>;

/// An opaque handle to an underlying resource in the `GraphicsServer`.
///
/// A handle can represent arbitrarily many different resources, and forms
/// the building blocks for any higher level APIs.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GraphicsHandle {
  pub(crate) id: usize,
}

/// An opaque context for graphics operations.
///
/// This context can be passed around the application and allows resources to refer back to
/// the originating graphics server.
#[derive(Clone)]
pub struct GraphicsContext {
  server: Rc<dyn GraphicsServer>,
}

impl GraphicsContext {
  /// Constructs a new graphics context for the given `GraphicsServer`.
  pub fn new(server: impl GraphicsServer + 'static) -> Self {
    Self { server: Rc::new(server) }
  }
}

/// Allows direct access to the backing `GraphicsServer`.
impl Deref for GraphicsContext {
  type Target = dyn GraphicsServer;

  fn deref(&self) -> &Self::Target {
    self.server.deref()
  }
}

/// A server for the underlying graphics subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level unsafe implementation abstraction.
pub unsafe trait GraphicsServer {
  // commands
  unsafe fn execute_command_buffer(&self, commands: &mut CommandBuffer) {
    while let Some(command) = commands.dequeue() {
      match command {
        Command::SetViewport(viewport) => self.set_viewport_size(viewport),
        Command::ClearColor(color) => self.clear_color_buffer(color),
        Command::ClearDepth => self.clear_depth_buffer(),
        _ => {}
      }
    }
  }

  // frame operations
  unsafe fn begin_frame(&self);
  unsafe fn end_frame(&self);

  // intrinsics
  unsafe fn set_viewport_size(&self, viewport: Viewport);
  unsafe fn clear_color_buffer(&self, color: Color);
  unsafe fn clear_depth_buffer(&self);
  unsafe fn flush_commands(&self);

  // buffers
  unsafe fn create_buffer(&self) -> GraphicsHandle;
  unsafe fn read_buffer_data(&self, buffer: GraphicsHandle) -> Vec<u8>;
  unsafe fn write_buffer_data(&self, buffer: GraphicsHandle, data: &[u8]);
  unsafe fn delete_buffer(&self, buffer: GraphicsHandle);

  // textures
  unsafe fn create_texture(&self) -> GraphicsHandle;
  unsafe fn write_texture_data(&self, texture: GraphicsHandle, data: &[u8]);
  unsafe fn delete_texture(&self, texture: GraphicsHandle);

  // shaders
  unsafe fn create_shader(&self) -> GraphicsHandle;
  unsafe fn delete_shader(&self, shader: GraphicsHandle);

  // meshes
  unsafe fn create_mesh(&self) -> GraphicsHandle;
}

/// Commands that can be enqueued in a `CommandBuffer` and replayed at a later date on the graphics
/// server or graphics pipeline.
pub enum Command {
  SetViewport(Viewport),
  ClearColor(Color),
  ClearDepth,
  DrawMesh {
    mesh: GraphicsHandle,
    vertex_buffer: GraphicsHandle,
    index_buffer: GraphicsHandle,
    topology: PrimitiveTopology,
    vertex_count: usize,
  },
}

/// A command buffer that can be used to issue instructions to the GPU.
pub struct CommandBuffer {
  commands: std::collections::VecDeque<Command>,
}

impl CommandBuffer {
  pub fn new() -> Self {
    Self {
      commands: std::collections::VecDeque::new()
    }
  }

  pub fn enqueue(&mut self, command: Command) {
    self.commands.push_back(command);
  }

  pub fn dequeue(&mut self) -> Option<Command> {
    self.commands.pop_front()
  }
}

/// A viewport for scissoring operations on a `GraphicsDevice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Viewport {
  pub width: usize,
  pub height: usize,
}

/// Represents the different topologies supported for a mesh.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTopology {
  Points,
  Lines,
  Triangles,
  Quads,
}

/// Represents the different blending modes for the graphics pipeline.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BlendingMode {
  None,
}

/// Texture wrapping modes modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WrapFunction {
  Clamp,
  Mirror,
}

/// Texture minify filter modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MinifyFilter {
  Nearest,
  Linear,
}

/// Texture magnify filter modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MagnifyFilter {
  Nearest,
  Linear,
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