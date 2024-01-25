//! Render target management and abstractions.
//!
//! Render targets allow for off-screen processing and rendering to a texture,
//! and form the basis of more complex render pipelines (deferred pipelines,
//! post-processing, etc).

use std::{cell::RefCell, rc::Rc};

use common::Rectangle;

use super::*;

/// Describes how to build a [`RenderTarget`].
///
/// A render target requires at least 1 color descriptor.
#[derive(Clone)]
pub struct RenderTargetDescriptor {
  pub color_attachment: RenderTextureDescriptor,
  pub depth_attachment: Option<RenderTextureDescriptor>,
  pub stencil_attachment: Option<RenderTextureDescriptor>,
}

/// Describes how to build a texture for use in a [`RenderTarget`].
#[derive(Clone)]
pub struct RenderTextureDescriptor {
  pub width: u32,
  pub height: u32,
  pub options: TextureOptions,
}

impl RenderTextureDescriptor {
  /// Converts this descriptor to a new [`Texture`].
  pub fn to_texture(&self, graphics: &GraphicsEngine) -> Result<Texture, TextureError> {
    Texture::with_options_and_size(graphics, &self.options, self.width, self.height, self.options.format)
  }
}

/// A render target is a collection of one or more buffers that can be rendered
/// to.
#[derive(Clone)]
pub struct RenderTarget {
  state: Rc<RefCell<RenderTargetState>>,
}

/// The inner state of a [`RenderTarget`].
struct RenderTargetState {
  id: TargetId,
  graphics: GraphicsEngine,
  color_attachment: Texture,
  depth_attachment: Option<Texture>,
  stencil_attachment: Option<Texture>,
}

impl RenderTarget {
  /// Creates a new [`RenderTarget`] on the GPU with the given attachments.
  pub fn new(graphics: &GraphicsEngine, descriptor: &RenderTargetDescriptor) -> Result<Self, TargetError> {
    let color_attachment = descriptor
      .color_attachment
      .to_texture(graphics)
      .map_err(|_| TargetError::FailedToBuildAttachments)?;

    let depth_attachment = descriptor
      .depth_attachment
      .as_ref()
      .and_then(|it| it.to_texture(graphics).ok());

    let stencil_attachment = descriptor
      .stencil_attachment
      .as_ref()
      .and_then(|it| it.to_texture(graphics).ok());

    Ok(Self {
      state: Rc::new(RefCell::new(RenderTargetState {
        id: graphics.target_create(
          color_attachment.id(),
          depth_attachment.as_ref().map(|it| it.id()),
          stencil_attachment.as_ref().map(|it| it.id()),
        )?,
        graphics: graphics.clone(),
        color_attachment,
        depth_attachment,
        stencil_attachment,
      })),
    })
  }

  /// Retrieves the [`TargetId`] of the underlying render target.
  pub fn id(&self) -> TargetId {
    self.state.borrow().id
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

    state
      .graphics
      .target_activate(state.id)
      .expect("Failed to activate render target");
  }

  /// Deactivates the [`RenderTarget`].
  pub fn deactivate(&self) {
    let state = self.state.borrow();

    state
      .graphics
      .target_set_default()
      .expect("Failed to deactivate render target");
  }

  /// Blits this render target to the given other target.
  pub fn blit_to(&self, other: &RenderTarget, filter: TextureFilter) {
    let state = self.state.borrow();

    let source_color = &state.color_attachment;
    let source = Rectangle::from_corner_points(0., 0., source_color.width() as f32, source_color.height() as f32);
    let dest_color = other.color_attachment();
    let dest = Rectangle::from_corner_points(0., 0., dest_color.width() as f32, dest_color.height() as f32);

    state
      .graphics
      .target_blit(state.id, other.id(), &source, &dest, filter)
      .expect("Failed to blit render target");
  }

  /// Blits this render target to the active display.
  pub fn blit_to_display(&self, filter: TextureFilter) {
    let state = self.state.borrow();
    let color = &state.color_attachment;

    let (width, height) = state.graphics.viewport_size();

    let source = Rectangle::from_corner_points(0., 0., color.width() as f32, color.height() as f32);
    let dest = Rectangle::from_corner_points(0., 0., width as f32, height as f32);

    state
      .graphics
      .target_blit_to_display(state.id, &source, &dest, filter)
      .expect("Failed to blit render target to display");
  }
}

impl Drop for RenderTargetState {
  fn drop(&mut self) {
    self
      .graphics
      .target_delete(self.id)
      .expect("Failed to delete render target");
  }
}
