//! Render target management and abstractions.
//!
//! Render targets allow for off-screen processing and rendering to a texture, and form
//! the basis of more complex render pipelines (deferred pipelines, post-processing, etc).

use std::cell::RefCell;
use std::rc::Rc;

use super::*;

/// Describes how to build a [`RenderTarget`].
#[derive(Clone)]
pub struct RenderTargetDescriptor {
  /// A render target requires at least 1 color attachment.
  pub color_attachment: RenderTextureDescriptor,
  pub depth_attachment: Option<RenderTextureDescriptor>,
  pub stencil_attachment: Option<RenderTextureDescriptor>,
}

/// Describes how to build a texture for use in a [`RenderTarget`].
#[derive(Clone)]
pub struct RenderTextureDescriptor {
  /// The width of the texture, in pixels.
  pub width: u32,
  /// The height of the texture, in pixels.
  pub height: u32,
  /// The options of the texture to be allocated.
  pub options: TextureOptions,
}

impl RenderTextureDescriptor {
  /// Converts this descriptor to a new [`Texture`].
  pub fn to_texture(&self, graphics: &GraphicsServer) -> Texture {
    let mut texture = Texture::with_options(graphics, &self.options);

    // allocate the memory ahead of time; RGBA8 format
    texture.initialize(self.width, self.height, self.options.format);

    texture
  }
}

/// A render target is a collection of one or more buffers that can be rendered to.
#[derive(Clone)]
pub struct RenderTarget {
  state: Rc<RefCell<RenderTargetState>>,
}

/// The inner state of a [`RenderTarget`].
struct RenderTargetState {
  graphics: GraphicsServer,
  handle: GraphicsHandle,
  color_attachment: Texture,
  depth_attachment: Option<Texture>,
  stencil_attachment: Option<Texture>,
}

impl RenderTarget {
  /// Creates a new blank [`RenderTarget`] on the GPU.
  pub fn new(graphics: &GraphicsServer, descriptor: &RenderTargetDescriptor) -> Self {
    // prepare attachments
    let color_attachment = descriptor.color_attachment.to_texture(graphics);

    let depth_attachment = descriptor.depth_attachment.as_ref().map(|it| it.to_texture(graphics));

    let stencil_attachment = descriptor.stencil_attachment.as_ref().map(|it| it.to_texture(graphics));

    let handle = graphics.create_render_target(
      color_attachment.handle(),
      depth_attachment.as_ref().map(|it| it.handle()),
      stencil_attachment.as_ref().map(|it| it.handle()),
    );

    Self {
      state: Rc::new(RefCell::new(RenderTargetState {
        graphics: graphics.clone(),
        handle,
        color_attachment,
        depth_attachment,
        stencil_attachment,
      })),
    }
  }

  /// Retrieves the color attachment for the target.
  pub fn color_attachment(&self) -> Texture {
    let state = self.state.borrow();

    state.color_attachment.clone()
  }

  /// Retrieves the depth attachment for the target.
  pub fn depth_attachment(&self) -> Option<Texture> {
    let state = self.state.borrow();

    state.depth_attachment.clone()
  }

  /// Retrieves the stencil attachment for the target.
  pub fn stencil_attachment(&self) -> Option<Texture> {
    let state = self.state.borrow();

    state.stencil_attachment.clone()
  }

  /// Activates the [`RenderTarget`].
  pub fn activate(&self) {
    let state = self.state.borrow();

    state.graphics.set_active_render_target(state.handle);
  }

  /// Deactivates the [`RenderTarget`].
  pub fn deactivate(&self) {
    let state = self.state.borrow();

    state.graphics.set_default_render_target();
  }

  /// Blits this render target to the given other target.
  pub fn blit_to(&self, other: &RenderTarget, filter: TextureFilter) {
    let state = self.state.borrow();

    let source_color = &state.color_attachment;
    let dest_color = other.color_attachment();

    let source = Rectangle::from_corner_points(0, 0, source_color.width() as i32, source_color.height() as i32);
    let dest = Rectangle::from_corner_points(0, 0, dest_color.width() as i32, dest_color.height() as i32);

    let graphics = &state.graphics;

    graphics.blit_render_target(state.handle, other.handle(), &source, &dest, filter);
  }

  /// Blits this render target to the active display.
  pub fn blit_to_display(&self, filter: TextureFilter) {
    let state = self.state.borrow();
    let color = &state.color_attachment;

    let (width, height) = state.graphics.get_viewport_size();

    let source = Rectangle::from_corner_points(0, 0, color.width() as i32, color.height() as i32);
    let dest = Rectangle::from_corner_points(0, 0, width as i32, height as i32);

    let graphics = &state.graphics;

    graphics.blit_render_target_to_display(state.handle, &source, &dest, filter);
  }
}

impl GraphicsResource for RenderTarget {
  /// Returns the underlying graphics handle of the [`RenderTarget`].
  fn handle(&self) -> GraphicsHandle {
    self.state.borrow().handle
  }
}

impl Drop for RenderTargetState {
  /// Deletes the [`RenderTarget`] from the GPU.
  fn drop(&mut self) {
    self.graphics.delete_render_target(self.handle);
  }
}
