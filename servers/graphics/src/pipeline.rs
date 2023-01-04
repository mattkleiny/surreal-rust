//! Rendering pipeline abstractions.

use surreal::scene::SceneGraph;

#[cfg(feature = "highdef")]
pub mod highdef;
#[cfg(feature = "universal")]
pub mod universal;

/// Allows a type to acts as input to the camera pipeline for a perspective or
/// orthographic rendering process.
pub trait RenderCamera {
  /// Retrieves the [`SceneGraph`] to be rendered.
  fn scene(&self) -> &SceneGraph;
}

/// Allows a type to participate in the rendering engine.
///
/// A pipeline is responsible for the discrete phases of the render process,
/// whether they are 'forward', 'deferred', or more exotic creations.
pub trait RenderPipeline {
  fn begin_frame(&self, frame: &RenderFrame);
  fn end_frame(&self, frame: &RenderFrame);
}

/// Encapsulates all of the details required to render a single frame in a
/// [`RenderPipeline`].
pub struct RenderFrame<'a, C = ()> {
  /// The [`RenderCamera`] for this particular frame.
  pub camera: &'a dyn RenderCamera,
  /// The context for this particular frame, which depends on the [`RenderPipeline`].
  pub context: &'a C,
}
