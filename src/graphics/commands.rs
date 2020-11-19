use super::{Buffer, Color, GraphicsDevice, PrimitiveTopology};

/// A buffer of commands to be executed on the GPU.
#[derive(Clone)]
pub struct CommandBuffer<'a> {
  commands: Vec<Command<'a>>,
}

impl<'a> CommandBuffer<'a> {
  /// Creates a new empty command buffer.
  pub fn new() -> Self {
    Self { commands: Vec::new() }
  }

  /// Enqueues a command to clear the active frame buffer.
  pub fn clear_frame_buffer(&mut self, color: Color) {
    self.commands.push(Command::ClearFrameBuffer(color));
  }

  /// Draws a mesh to the active frame buffer.
  pub fn draw_mesh(&mut self, topology: PrimitiveTopology, vertex_buffer: &'a Buffer, index_buffer: &'a Buffer, vertex_count: usize) {
    self.commands.push(Command::DrawMesh {
      topology,
      vertex_buffer,
      index_buffer,
      vertex_count,
    })
  }

  /// Executes the command buffer's commands on the given graphics device.
  pub fn execute(&mut self, device: &mut impl GraphicsDevice) {
    for command in self.commands.drain(..) {
      match command {
        Command::ClearFrameBuffer(color) => {
          device.clear_frame_buffer(color);
        }
        Command::DrawMesh { topology, vertex_buffer, index_buffer, vertex_count } => {
          device.draw_mesh(topology, vertex_buffer, index_buffer, vertex_count);
        }
      }
    }
  }
}

/// Represents a single command in a `CommandBuffer`.
#[derive(Clone)]
enum Command<'a> {
  /// Clears the active frame buffer.
  ClearFrameBuffer(Color),
  /// Draws a mesh to the active frame buffer.
  DrawMesh {
    topology: PrimitiveTopology,
    vertex_buffer: &'a Buffer,
    index_buffer: &'a Buffer,
    vertex_count: usize,
  },
}