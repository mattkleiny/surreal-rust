//! Game framework for Surreal.
//!
//! Bootstrapping and other framework systems for common projects.

pub use ecs::*;

use crate::assets::AssetManager;
use crate::graphics::{ImageLoader, MaterialLoader, ShaderProgramLoader, TextureLoader, TextureOptions};
use crate::platform::{Platform, PlatformHost};
use crate::utilities::{Clock, GameTime};

mod ecs;

/// The context for bootstrapping a game.
pub struct Game<P> where P: Platform {
  pub host: P::Host,
  pub assets: AssetManager,
}

/// The context for a single tick of the game loop.
pub struct GameTick<'a, P> where P: Platform {
  pub host: &'a P::Host,
  pub time: GameTime,
  is_running: bool,
}

impl<P> Game<P> where P: Platform {
  /// Starts a new game with the given platform.
  pub fn start(platform: P, mut setup: impl FnMut(Game<P>)) {
    let mut game = Game {
      host: platform.create_host(),
      assets: AssetManager::new(),
    };

    // set-up default asset loaders
    let host: &P::Host = &game.host;
    let graphics = host.graphics();

    game.assets.add_loader(ImageLoader {
      format: None
    });

    game.assets.add_loader(TextureLoader {
      server: graphics.clone(),
      options: TextureOptions::default(),
    });

    game.assets.add_loader(ShaderProgramLoader {
      server: graphics.clone()
    });

    game.assets.add_loader(MaterialLoader {
      server: graphics.clone()
    });

    setup(game);
  }

  /// Runs the game loop in a variable step fashion.
  pub fn run_variable_step(&mut self, mut tick: impl FnMut(&mut GameTick<P>)) {
    let mut timer = Clock::new();

    self.host.run(move |host| {
      let mut context = GameTick {
        host,
        time: GameTime {
          delta_time: timer.tick(),
          total_time: timer.total_time(),
        },
        is_running: true,
      };

      tick(&mut context);

      if !context.is_running {
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
