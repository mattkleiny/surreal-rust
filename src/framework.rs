//! Game framework for Surreal.
//!
//! Bootstrapping and other framework systems for common projects.

pub use ecs::*;

use crate::assets::AssetManager;
use crate::graphics::{BitmapFontLoader, ImageLoader, MaterialLoader, ShaderProgramLoader, TextureLoader, TextureOptions};
use crate::platform::{Platform, PlatformHost};
use crate::utilities::{Clock, GameTime};

mod ecs;

// TODO: screen management
// TODO: plugin management (profiler, console, etc)
// TODO: better rendering pipeline support

/// The context for bootstrapping a game.
pub struct Game<P> where P: Platform {
  pub host: P::Host,
}

/// The context for a single tick of the game loop.
pub struct GameTick<'a, P> where P: Platform {
  pub host: &'a mut P::Host,
  pub time: GameTime,
  is_running: bool,
}

impl<P> Game<P> where P: Platform {
  /// Starts a new game with the given platform.
  pub fn start(platform: P, mut setup: impl FnMut(Game<P>, &mut AssetManager)) {
    #[cfg(feature = "profiling")] {
      puffin::set_scopes_on(true);
    }

    // set-up core host
    let game = Game { host: platform.create_host() };
    let host: &P::Host = &game.host;
    let graphics = host.graphics();

    // set-up asset manager
    let mut assets = AssetManager::new();

    assets.add_loader(BitmapFontLoader {});
    assets.add_loader(ImageLoader { format: None });
    assets.add_loader(TextureLoader { server: graphics.clone(), options: TextureOptions::default() });
    assets.add_loader(ShaderProgramLoader { server: graphics.clone() });
    assets.add_loader(MaterialLoader { server: graphics.clone() });

    // set-up lua scripting
    #[cfg(feature = "scripting")] {
      use crate::scripting::*;

      // configure lua script backend and default script loader
      let scripting = LuaScriptBackend::new();
      assets.add_loader(ScriptLoader { server: scripting.clone() });
    }

    setup(game, &mut assets);
  }

  /// Runs the game loop in a variable step fashion.
  pub fn run_variable_step(&mut self, mut main_loop: impl FnMut(&mut GameTick<P>)) {
    let mut timer = Clock::new();

    self.host.run(move |host| {
      puffin::GlobalProfiler::lock().new_frame();

      let mut tick = GameTick {
        host,
        time: GameTime {
          delta_time: timer.tick(),
          total_time: timer.total_time(),
        },
        is_running: true,
      };

      main_loop(&mut tick);

      if !tick.is_running {
        host.exit();
      }
    });
  }
}

impl<'a, P> GameTick<'a, P> where P: Platform {
  /// Exits the game at the end of the frame.
  pub fn exit(&mut self) {
    self.is_running = false;
  }
}
