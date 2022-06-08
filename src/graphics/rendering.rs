//! Rendering abstractions and pipelines.
//!
//! This is a series of components designed to make it simpler to build more complex render
//! pipelines than using the 'material', 'mesh', 'render targets' etc do alone.

use crate::collections::AnyMap;
use std::collections::VecDeque;

use super::*;

/// A command buffer encodes a set of instructions to be replayed against the graphics server.
///
/// Command buffers decouple the order of instructions from the executino of those instructions
/// and allow for collection of commands from across multiple threads and workers.
#[derive(Default)]
pub struct CommandBuffer {
  commands: VecDeque<Command>,
}

/// Encodes a single command in the command buffer.
enum Command {
  // TODO: how to handle
  ClearColor(Color),
  ClearDepth,
  SetRenderTarget(RenderTarget),
  SetRenderTargetToDisplay,
  Blit(RenderTarget, RenderTarget),
  BlitToDisplay(RenderTarget),
  // RenderMesh(Mesh),
}

impl CommandBuffer {
  /// Creates a new command buffer.
  pub fn new() -> Self {
    Self {
      commands: VecDeque::new(),
    }
  }

  /// Clears the active color buffer.
  pub fn clear_color_buffer(&mut self, color: Color) {
    self.commands.push_back(Command::ClearColor(color));
  }

  /// Clears the active depth buffer.
  pub fn clear_depth_buffer(&mut self) {
    self.commands.push_back(Command::ClearDepth);
  }

  /// Sets the active render target to the given target.
  pub fn set_render_target(&mut self, target: &RenderTarget) {
    self.commands.push_back(Command::SetRenderTarget(target.clone()));
  }

  /// Sets the active render target to the default display.
  pub fn set_render_target_to_display(&mut self) {
    self.commands.push_back(Command::SetRenderTargetToDisplay);
  }

  /// Blits between two given render targets.
  pub fn blit_to(&mut self, from: &RenderTarget, to: &RenderTarget) {
    self.commands.push_back(Command::Blit(from.clone(), to.clone()));
  }

  /// Blits the given render target to the active display.
  pub fn blit_to_display(&mut self, target: &RenderTarget) {
    self.commands.push_back(Command::BlitToDisplay(target.clone()));
  }

  /// Executes the commands in the command buffer and clears it.
  pub fn flush(&mut self, graphics: &GraphicsServer) {
    while let Some(command) = self.commands.pop_front() {
      self.execute_command(command, graphics);
    }
  }

  /// Executes the given command.
  fn execute_command(&mut self, command: Command, _graphics: &GraphicsServer) {
    match command {
      Command::ClearColor(_) => todo!(),
      Command::ClearDepth => todo!(),
      Command::SetRenderTarget(_) => todo!(),
      Command::SetRenderTargetToDisplay => todo!(),
      Command::Blit(_, _) => todo!(),
      Command::BlitToDisplay(_) => todo!(),
    }
  }
}

/// Allows an object to be rendered via a [`RenderManager`].
///
/// Requires that the manager is configured for the associated context.
pub trait Renderable<C: RenderContext> {
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
  fn create(&self, graphics: &GraphicsServer) -> Self::Context;
}

/// A manager for `RenderContext`s.
///
/// A [`RenderContext`] encodes all of the required details for textures,
/// materials, render targets, shaders, necessary in a single invocation of some
/// rendering state.
pub struct RenderContextManager {
  graphics: GraphicsServer,
  contexts: AnyMap,
}

impl RenderContextManager {
  /// Creates a new render manager.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      graphics: graphics.clone(),
      contexts: AnyMap::new(),
    }
  }

  /// Configures the manager with the given context.
  pub fn configure<D: RenderContextDescriptor>(&mut self, descriptor: D) {
    self.contexts.insert(descriptor.create(&self.graphics));
  }

  /// Renders the given object via the associated context.
  pub fn render<R: Renderable<C>, C: RenderContext>(&mut self, renderable: &R) {
    self.with(|context| {
      renderable.render(context);
    });
  }

  /// Acquires a context for the given descriptor.
  ///
  /// If the context cannot be acquired, the body will not be run.
  pub fn with<C: RenderContext>(&mut self, body: impl FnOnce(&mut C)) {
    if let Some(context) = self.contexts.get_mut::<C>() {
      context.on_before_with();
      body(context);
      context.on_after_with();
    }
  }

  /// Releases the given context from the manager.
  pub fn release<C: RenderContext>(&mut self) {
    self.contexts.remove::<C>();
  }

  /// Clears all contexts from the manager, resetting it to a clean state.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}

/// A manager for `RenderPass`es.
/// 
/// Render passes are executed based on their `RenderPass::order()`.
#[derive(Default)]
pub struct RenderPassManager {
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

impl RenderPassManager {
  /// Creates a new renderer.
  pub fn new() -> Self {
    Self {
      passes: Vec::new(),
      commands: CommandBuffer::new(),
    }
  }

  /// Adds a `RenderPass` to the renderer.
  pub fn add_pass(&mut self, pass: impl RenderPass + 'static) {
    self.passes.push(Box::new(pass));
    self.passes.sort_by_key(|pass| pass.order());
  }

  /// Renders a single frame to the given `GraphicsServer`.
  pub fn render(&mut self, graphics: &GraphicsServer) {
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

    commands.flush(graphics);
  }
}