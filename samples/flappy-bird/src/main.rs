//! A simple example of a Flappy Bird game using surreal and hecs.

use hecs::{EntityBuilder, World};
use surreal::{
  backends::sdl::{Window, WindowSettings},
  common::{DeltaClock, StringName},
  graphics::{Color, Color32, GraphicsEngine, RenderQueue},
};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Flappy Bird".to_string(),
    width: 800,
    height: 600,
    vsync_enabled: true,
    icon: None,
  })
  .unwrap();

  let graphics = GraphicsEngine::opengl(&window);

  let mut game = Game::new();
  let mut queue = RenderQueue::new();

  while window.update() {
    game.update();
    game.draw(&mut queue);

    queue.flush(&graphics).expect("Failed to flush render queue");

    window.present();
  }
}

struct Game {
  world: World,
  clock: DeltaClock,
  sprite_cache: SpriteCache,
}

struct Position {
  x: f32,
  y: f32,
}

struct Velocity {
  x: f32,
  y: f32,
}

struct Sprite {
  texture: StringName,
  tint: Color32,
}

struct SpriteCache {}

impl Game {
  pub fn new() -> Self {
    let mut world = World::new();

    // create entities
    for _ in 0..10 {
      let mut builder = EntityBuilder::default();

      builder.add(Position { x: 0.0, y: 0.0 });
      builder.add(Velocity { x: 1.0, y: 1.0 });
      builder.add(Sprite {
        texture: "bird".into(),
        tint: Color32::WHITE,
      });

      world.spawn(builder.build());
    }

    Self {
      world,
      clock: DeltaClock::new(),
      sprite_cache: SpriteCache {},
    }
  }

  pub fn update(&mut self) {
    let delta_time = self.clock.tick();

    // update positions
    for (_, (position, velocity)) in self.world.query_mut::<(&mut Position, &mut Velocity)>() {
      position.x += velocity.x * delta_time;
      position.y += velocity.y * delta_time;
    }
  }

  pub fn draw(&self, queue: &mut RenderQueue) {
    queue.clear_color_buffer(Color::BLACK);

    for (_, (_position, _sprite)) in &mut self.world.query::<(&Position, &Sprite)>() {
      // queue
    }
  }
}
