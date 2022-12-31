//! Common utilities for the graphics integration tests.

use surreal_core::engine::{Configuration, Engine};
use surreal_core::graphics::GraphicsServer;

/// Bootstraps the [`Engine`] for an integration test.
pub fn bootstrap(body: impl Fn(&GraphicsServer)) {
  let configuration = Configuration {
    title: "Surreal Integration Test",
    size: (1280, 1024),
    is_window_visible: false,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    body(&engine.graphics);
  });
}
