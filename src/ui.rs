//! User interface support.
//!
//! Internally we integrate the excellent egui library into the core engine.

use std::collections::HashMap;

use crate::graphics::*;

/// A shader program to use for egui UI rendering.
const SHADER_UI_STANDARD: &'static str = include_str!("../assets/shaders/ui-standard.glsl");

/// A provider for egui raw input.
pub trait RawInputProvider {
  /// Retrieves raw input for this frame.
  fn get_raw_input(&self) -> &egui::RawInput;
}

/// Describes how to set-up a `UserInterfaceContext` for egui.
pub struct UserInterfaceContextDescriptor;

impl RenderContextDescriptor for UserInterfaceContextDescriptor {
  type Context = UserInterfaceContext;

  fn create(&self, graphics: &GraphicsServer) -> Self::Context {
    let shader = ShaderProgram::from_string(&graphics, SHADER_UI_STANDARD).unwrap();

    Self::Context {
      graphics: graphics.clone(),
      material: Material::new(graphics, &shader),
      textures: HashMap::new(),
      context: egui::Context::default(),
    }
  }
}

/// A context for immediate mode user interface rendering via `egui`.
pub struct UserInterfaceContext {
  graphics: GraphicsServer,
  material: Material,
  textures: HashMap<egui::TextureId, Texture>,
  context: egui::Context,
}

impl UserInterfaceContext {
  pub fn run(&mut self, input: &impl RawInputProvider, body: impl FnMut(&egui::Context)) {
    // TODO: apply full output from egui to the graphics server

    let raw_input = input.get_raw_input().clone();
    let full_output = self.context.run(raw_input, body);
  }
}

impl RenderContext for UserInterfaceContext {
  fn on_before_with(&mut self) {}
  fn on_after_with(&mut self) {}
}