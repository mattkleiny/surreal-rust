//! User interface support.
//!
//! Internally we integrate the excellent egui library into the core engine.

use std::collections::HashMap;

use crate::graphics::*;

const SHADER_UI_STANDARD: &'static str = include_str!("../assets/shaders/sprite-standard.glsl");

pub struct EguiContextDescriptor;

impl RenderContextDescriptor for EguiContextDescriptor {
  type Context = EguiContext;

  fn create(&self, server: &GraphicsServer) -> Self::Context {
    let shader = ShaderProgram::from_string(&server, SHADER_UI_STANDARD).unwrap();

    Self::Context {
      server: server.clone(),
      material: Material::new(server, &shader),
      textures: HashMap::new(),
    }
  }
}

pub struct EguiContext {
  server: GraphicsServer,
  material: Material,
  textures: HashMap<egui::TextureId, Texture>,
}

impl EguiContext {
  pub fn run(&mut self, body: impl FnMut(&egui::Context)) {
    todo!()
  }
}

impl RenderContext for EguiContext {
  fn on_before_with(&mut self) {}
  fn on_after_with(&mut self) {}
}