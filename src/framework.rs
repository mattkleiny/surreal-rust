//! Game framework for Surreal.
//!
//! Bootstrapping and other framework systems for common projects.

pub use ecs::*;

use crate::assets::AssetManager;
use crate::graphics::{
  BitmapFontLoader, ImageLoader, MaterialLoader, ShaderProgramLoader, TextureLoader, TextureOptions,
};
use crate::platform::{Platform, PlatformHost};
use crate::utilities::{Clock, GameTime};

pub use events::*;

mod ecs;
mod events;

// TODO: screen management
// TODO: plugin management (profiler, console, etc)
// TODO: better rendering pipeline support

/// The context for bootstrapping a game.
pub struct Game<P: Platform> {
  pub host: P::Host,
}

/// The context for a single tick of the game loop.
pub struct GameTick {
  pub time: GameTime,
  is_exiting: bool,
}

impl GameTick {
  /// Exits the game at the end of the frame.
  pub fn exit(&mut self) {
    self.is_exiting = true;
  }
}

impl<P: Platform> Game<P> {
  /// Starts a new game with the given platform.
  pub fn start(platform: P, mut setup: impl FnMut(Game<P>, &mut AssetManager)) {
    profiling::register_thread!("Main Thread");

    // set-up core host
    let game = Game {
      host: platform.create_host(),
    };

    let host: &P::Host = &game.host;
    let graphics = host.graphics();

    // set-up asset manager
    let mut assets = AssetManager::new();

    assets.add_loader(BitmapFontLoader {});
    assets.add_loader(ImageLoader { format: None });

    assets.add_loader(TextureLoader {
      server: graphics.clone(),
      options: TextureOptions::default(),
    });

    assets.add_loader(ShaderProgramLoader {
      server: graphics.clone(),
    });

    assets.add_loader(MaterialLoader {
      server: graphics.clone(),
    });

    setup(game, &mut assets);
  }

  /// Runs the game loop in a variable step fashion.
  pub fn run_variable_step(self, mut body: impl FnMut(&mut P::Host, &mut GameTick)) {
    use winit::event_loop::ControlFlow;

    let mut timer = Clock::new();

    self.host.run(move |host, control_flow| {
      // capture timing information
      let mut tick = GameTick {
        time: GameTime {
          delta_time: timer.tick(),
          total_time: timer.total_time(),
        },
        is_exiting: false,
      };

      // run main loop
      body(host, &mut tick);

      if tick.is_exiting {
        *control_flow = ControlFlow::Exit;
      }

      profiling::finish_frame!();
    });
  }
}
