//! Common utilities for the graphics integration tests.

use surreal_core::engine::{Engine, EngineConfig};
use surreal_core::graphics::GraphicsServer;

/// Bootstraps the [`Engine`] for an integration test.
pub fn bootstrap(body: impl Fn(&GraphicsServer)) {
  if std::env::var("HEADLESS_ENVIRONMENT").is_ok() {
    return; // we can't bootstrap in a headless test environment
  }

  let configuration = EngineConfig {
    title: "Surreal Integration Test".to_string(),
    size: (1280, 1024),
    is_window_visible: false,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    body(&engine.graphics);
  });
}
