use crate::collections::AnyMap;
use std::collections::VecDeque;

use super::*;

/// A command buffer encodes a set of instructions to be replayed against the graphics server.
///
/// Command buffers decouple the order of instructions from the executino of those instructions
/// and allow for collection of commands from across multiple threads and workers.
pub struct CommandBuffer {
  server: GraphicsServer,
  commands: VecDeque<Command>,
}

/// Encodes a single command in the command buffer.
enum Command {
  ClearColor(Color),
  ClearDepth,
}

impl CommandBuffer {
  /// Creates a new command buffer.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      server: server.clone(),
      commands: VecDeque::new(),
    }
  }

  pub fn clear_color_buffer(&mut self, color: Color) {
    self.commands.push_back(Command::ClearColor(color));
  }

  pub fn clear_depth_buffer(&mut self) {
    self.commands.push_back(Command::ClearDepth);
  }

  /// Executes the commands in the command buffer and clears it.
  pub fn flush(&mut self) {
    while let Some(command) = self.commands.pop_front() {
      self.execute_command(command);
    }
  }

  /// Executes the given command.
  fn execute_command(&mut self, command: Command) {
    match command {
      Command::ClearColor(color) => {
        self.server.clear_color_buffer(color);
      }
      Command::ClearDepth => {
        self.server.clear_depth_buffer();
      }
    }
  }
}

/// A renderer based on render passes and the command buffer.
pub struct Renderer {
  passes: Vec<Box<dyn RenderPass>>,
  commands: CommandBuffer,
}

/// Represents a single render pass in a renderer.
///
/// Render passes are executed in order to create a single frame.
pub trait RenderPass {
  /// The order in which this pass should evaluate.
  ///
  /// Higher values are evaluated last.
  fn order(&self) -> usize;

  fn begin_frame(&mut self, commands: &mut CommandBuffer);
  fn render_frame(&mut self, commands: &mut CommandBuffer);
  fn end_frame(&mut self, commands: &mut CommandBuffer);
}

impl Renderer {
  /// Creates a new renderer.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      passes: Vec::new(),
      commands: CommandBuffer::new(server),
    }
  }

  /// Adds a render pass to the renderer.
  pub fn add_pass(&mut self, pass: impl RenderPass + 'static) {
    self.passes.push(Box::new(pass));
    self.passes.sort_by_key(|pass| pass.order());
  }

  /// Renders a single frame.
  pub fn render(&mut self) {
    let commands = &mut self.commands;

    for pass in &mut self.passes {
      pass.begin_frame(commands);
    }

    for pass in &mut self.passes {
      pass.render_frame(commands);
    }

    for pass in &mut self.passes {
      pass.end_frame(commands);
    }

    commands.flush();
  }
}

/// Allows an object to be rendered via a [`RenderManager`].
///
/// Requires that the manager is configured for the associated context.
pub trait Renderable<C>
where
  C: RenderContext,
{
  /// Renders this object via the associated [`RenderContext`].
  fn render(&self, context: &mut C);
}

/// A context for rendering operations.
///
/// A context contains the state useful for a particular kind of rendering operation, and also
/// exposes some basic lifecycle methods.
pub trait RenderContext: Sized + 'static {
  fn on_initialize(&mut self) {}
  fn on_before_with(&mut self) {}
  fn on_after_with(&mut self) {}
}

/// Describes how to build a [`RenderContext`] .
///
/// A descriptor is a factory for a render context, and contain configuration and shared data
/// that is usable in the creation of the context itself.
pub trait RenderContextDescriptor {
  /// The type of context that will be created by this descriptor.
  type Context: RenderContext;

  /// Creates the associated context.
  fn create(&self, server: &GraphicsServer) -> Self::Context;
}

/// A renderer is responsible for rendering a scene.
///
/// The render manages a set of [`RenderContext`] s which include all the required details for
/// textures, materials, render targets, shaders, etc.
///
/// Each context can be acquired and utilized via the `with` method.
pub struct RenderManager {
  server: GraphicsServer,
  contexts: AnyMap,
}

impl RenderManager {
  /// Creates a new render manager.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      server: server.clone(),
      contexts: AnyMap::new(),
    }
  }

  /// Configures the manager with the given context.
  pub fn configure<D>(&mut self, descriptor: D)
  where
    D: RenderContextDescriptor,
  {
    self.contexts.insert(descriptor.create(&self.server));
  }

  /// Renders the given object via the associated context.
  pub fn render<R, C>(&mut self, renderable: &R)
  where
    R: Renderable<C>,
    C: RenderContext,
  {
    self.with(|context| {
      renderable.render(context);
    });
  }

  /// Acquires a context for the given descriptor.
  ///
  /// If the context cannot be acquired, the body will not be run.
  pub fn with<C>(&mut self, body: impl FnOnce(&mut C) -> ())
  where
    C: RenderContext,
  {
    if let Some(context) = self.contexts.get_mut::<C>() {
      context.on_before_with();
      body(context);
      context.on_after_with();
    }
  }

  /// Releases the given context from the manager.
  pub fn release<C>(&mut self)
  where
    C: RenderContext,
  {
    self.contexts.remove::<C>();
  }

  /// Clears all contexts from the manager, resetting it to a clean state.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}
