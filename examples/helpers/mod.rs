//! Common tools for examples.

use surreal::prelude::*;

/// Bootstraps an example for the engine
pub fn run_example(name: &'static str, body: impl FnOnce(Engine, AssetManager)) {
  let configuration = Configuration {
    title: name,
    size: (1280, 1024),
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    body(engine, assets);
  });
}
