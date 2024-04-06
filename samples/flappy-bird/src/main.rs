//! A simple example of a Flappy Bird game using surreal and hecs.

use hecs::World;
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
  let mut queue = RenderQueue::new();

  let mut game = Game::new();

  game.initialize();

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

struct Player {}

struct Obstacle {}

impl Game {
  pub fn new() -> Self {
    Self {
      world: World::new(),
      clock: DeltaClock::new(),
    }
  }

  pub fn initialize(&mut self) {
    // spawn the player
    self.world.spawn((
      Position { x: 0.0, y: 0.0 },
      Velocity { x: 0.0, y: 0.0 },
      Sprite {
        texture: "sprites/player.png".into(),
        tint: Color32::WHITE,
      },
      Player {},
    ));

    // spawn the first obstacle
    self.world.spawn((
      Position { x: 0.0, y: 0.0 },
      Sprite {
        texture: "sprites/obstacle.png".into(),
        tint: Color32::WHITE,
      },
      Obstacle {},
    ));
  }

  pub fn update(&mut self) {
    let delta_time = self.clock.tick();

    self.update_physics(delta_time);
    self.update_player();
  }

  fn update_physics(&mut self, delta_time: f32) {
    const GRAVITY: f32 = 9.8;

    for (_, (position, velocity)) in self.world.query_mut::<(&mut Position, &mut Velocity)>() {
      position.x += velocity.x * delta_time;
      position.y += velocity.y * delta_time;

      velocity.y += GRAVITY * delta_time;
    }
  }

  fn update_player(&mut self) {
    for (_, (_velocity, _player)) in self.world.query_mut::<(&mut Velocity, &Player)>() {
      // TODO: player input
    }
  }

  pub fn draw(&self, queue: &mut RenderQueue) {
    queue.clear_color_buffer(Color::BLACK);

    self.draw_sprites(queue);
  }

  fn draw_sprites(&self, _queue: &mut RenderQueue) {
    for (_, (_position, _sprite)) in &mut self.world.query::<(&Position, &Sprite)>() {
      // queue
    }
  }
}
