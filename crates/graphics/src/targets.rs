//! Render target management and abstractions.
//!
//! Render targets allow for off-screen processing and rendering to a texture,
//! and form the basis of more complex render pipelines (deferred pipelines,
//! post-processing, etc.).

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

impl RenderTargetDescriptor {
  /// Creates a new descriptor with the given size.
  pub fn with_size(&self, width: u32, height: u32) -> Self {
    Self {
      color_attachment: self.color_attachment.with_size(width, height),
      depth_attachment: self.depth_attachment.as_ref().map(|it| it.with_size(width, height)),
      stencil_attachment: self.stencil_attachment.as_ref().map(|it| it.with_size(width, height)),
      ..self.clone()
    }
  }
}

impl RenderTextureDescriptor {
  /// Creates a new descriptor with the given size.
  pub fn with_size(&self, width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      ..self.clone()
    }
  }

  /// Converts this descriptor to a new [`Texture`].
  pub fn to_texture(&self) -> Result<Texture, TextureError> {
    Texture::new(self.width, self.height, &self.options)
  }
}

/// A render target is a collection of one or more buffers that rendered.
#[derive(Clone)]
pub struct RenderTarget {
  state: GraphicsCell<RenderTargetState>,
}

/// The inner state of a [`RenderTarget`].
struct RenderTargetState {
  id: TargetId,
  color_attachment: Texture,
  depth_attachment: Option<Texture>,
  stencil_attachment: Option<Texture>,
}

impl RenderTarget {
  /// Creates a new [`RenderTarget`] on the GPU with the given attachments.
  pub fn new(target_descriptor: &RenderTargetDescriptor) -> Result<Self, TargetError> {
    let color_attachment = target_descriptor
      .color_attachment
      .to_texture()
      .map_err(|_| TargetError::FailedToBuildAttachments)?;

    let depth_attachment = target_descriptor
      .depth_attachment
      .as_ref()
      .and_then(|it| it.to_texture().ok());

    let stencil_attachment = target_descriptor
      .stencil_attachment
      .as_ref()
      .and_then(|it| it.to_texture().ok());

    Ok(Self {
      state: GraphicsCell::new(RenderTargetState {
        id: graphics().target_create(
          color_attachment.id(),
          depth_attachment.as_ref().map(|it| it.id()),
          stencil_attachment.as_ref().map(|it| it.id()),
        )?,
        color_attachment,
        depth_attachment,
        stencil_attachment,
      }),
    })
  }

  /// Retrieves the [`TargetId`] of the underlying render target.
  pub fn id(&self) -> TargetId {
    self.state.read().id
  }

  /// Retrieves the color attachment for the target.
  pub fn color_attachment(&self) -> Texture {
    self.state.read().color_attachment.clone()
  }

  /// Retrieves the depth attachment for the target.
  pub fn depth_attachment(&self) -> Option<Texture> {
    self.state.read().depth_attachment.clone()
  }

  /// Retrieves the stencil attachment for the target.
  pub fn stencil_attachment(&self) -> Option<Texture> {
    self.state.read().stencil_attachment.clone()
  }

  /// Activates the [`RenderTarget`].
  pub fn activate(&self) {
    graphics()
      .target_activate(self.id())
      .expect("Failed to activate render target");
  }

  /// Deactivates the [`RenderTarget`].
  pub fn deactivate(&self) {
    graphics()
      .target_set_default()
      .expect("Failed to deactivate render target");
  }

  /// Blits this render target to the active target.
  pub fn blit_to_active(&self, filter: TextureFilter) {
    graphics()
      .target_blit_to_active(self.id(), None, None, filter)
      .expect("Failed to blit render target to display");
  }
}

impl Drop for RenderTargetState {
  fn drop(&mut self) {
    graphics()
      .target_delete(self.id)
      .expect("Failed to delete render target");
  }
}
