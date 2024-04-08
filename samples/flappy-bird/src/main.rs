//! A simple Flappy Bird game using Surreal.

use hecs::World;
use surreal::{
  backends::sdl::{Window, WindowSettings},
  common::{vec2, DeltaClock, FastHashMap, Mat4, StringName, Vec2},
  graphics::{
    Color, Color32, GraphicsEngine, Material, RenderQueue, SpriteBatch, SpriteOptions, Texture, TextureRegion,
    SHADER_SPRITE_STANDARD,
  },
  input::{KeyboardDevice, VirtualKey},
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
  let shader = SHADER_SPRITE_STANDARD.to_program(&graphics).unwrap();
  let mut material = Material::new(&graphics, &shader);
  let mut queue = RenderQueue::new();
  let mut game = Game::new(&graphics);

  material.set_uniform("u_projectionView", Mat4::IDENTITY);

  game.initialize();

  while window.update() {
    game.update(&window);
    game.draw(&graphics);

    queue.flush(&graphics).expect("Failed to flush render queue");

    window.present();
  }
}

struct Game {
  world: World,
  clock: DeltaClock,
  sprites: SpriteQueue,
}

struct Position(Vec2);
struct Velocity(Vec2);

struct Sprite {
  texture: StringName,
  tint: Color32,
}

struct SpriteQueue {
  graphics: GraphicsEngine,
  sprite_by_name: FastHashMap<StringName, TextureRegion>,
  sprite_batch: SpriteBatch,
}

struct Player {}

struct Obstacle {}

impl SpriteQueue {
  pub fn new(graphics: &GraphicsEngine) -> Self {
    Self {
      graphics: graphics.clone(),
      sprite_by_name: FastHashMap::default(),
      sprite_batch: SpriteBatch::new(graphics).unwrap(),
    }
  }

  pub fn draw_sprite(&mut self, name: StringName, position: Vec2, tint: Color32) {
    match self.sprite_by_name.get(&name) {
      Some(region) => {
        self.sprite_batch.draw_sprite(region, &SpriteOptions {
          position,
          color: tint,
          ..Default::default()
        });
      }
      None => {
        let path = format!("assets/sprites/{}", name);
        let texture = Texture::from_path(&self.graphics, path).unwrap();

        let region = TextureRegion::new(&texture);

        self.sprite_batch.draw_sprite(&region, &SpriteOptions {
          position,
          color: tint,
          ..Default::default()
        });

        self.sprite_by_name.insert(name, region);
      }
    }
  }

  pub fn flush(&mut self) {
    self.sprite_batch.flush();
  }
}

impl Game {
  pub fn new(graphics: &GraphicsEngine) -> Self {
    Self {
      world: World::new(),
      clock: DeltaClock::new(),
      sprites: SpriteQueue::new(graphics),
    }
  }

  pub fn initialize(&mut self) {
    // spawn the player
    self.world.spawn((
      Position(Vec2::ZERO),
      Velocity(Vec2::ZERO),
      Sprite {
        texture: "player.png".into(),
        tint: Color32::WHITE,
      },
      Player {},
    ));

    // spawn the first obstacle
    self.world.spawn((
      Position(Vec2::ZERO),
      Sprite {
        texture: "obstacle.png".into(),
        tint: Color32::WHITE,
      },
      Obstacle {},
    ));
  }

  pub fn update(&mut self, keyboard: &dyn KeyboardDevice) {
    let delta_time = self.clock.tick();

    self.update_physics(delta_time);
    self.update_player(keyboard);
  }

  fn update_physics(&mut self, delta_time: f32) {
    const GRAVITY: Vec2 = vec2(0.0, -9.8);

    for (_, (position, velocity)) in self.world.query_mut::<(&mut Position, &mut Velocity)>() {
      position.0 += velocity.0 * delta_time;
      velocity.0 += GRAVITY * delta_time;
    }
  }

  fn update_player(&mut self, keyboard: &dyn KeyboardDevice) {
    const JUMP_VELOCITY: Vec2 = vec2(0.0, 5.0);

    for (_, (velocity, _)) in self.world.query_mut::<(&mut Velocity, &Player)>() {
      if keyboard.is_key_down(VirtualKey::Space) {
        velocity.0 += JUMP_VELOCITY;
      }
    }
  }

  pub fn draw(&mut self, graphics: &GraphicsEngine) {
    graphics.clear_color_buffer(Color::WHITE);

    self.draw_sprites();
    self.sprites.flush();
  }

  fn draw_sprites(&mut self) {
    for (_, (position, sprite)) in &mut self.world.query::<(&Position, &Sprite)>() {
      self.sprites.draw_sprite(sprite.texture, position.0, sprite.tint);
    }
  }
}
