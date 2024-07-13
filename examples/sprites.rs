use surreal::{backends::sdl::*, common::*, graphics::*};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Sprites".to_string(),
    ..Default::default()
  })
  .expect("Failed to build main window");

  GraphicsServer::install(OpenGLGraphicsBackend::new(&window));

  let mut batch = SpriteBatch::new().expect("Failed to create sprite batch");
  let texture = Texture::from_path("assets/sprites/bunny.png").expect("Failed to load texture");
  let mut material = SHADER_SPRITE_STANDARD.to_material().expect("Failed to load material");

  material.set_blend_state(BlendState::Enabled {
    source: BlendFactor::SourceAlpha,
    destination: BlendFactor::OneMinusSourceAlpha,
  });

  material.set_uniform(
    PROJECTION_VIEW,
    &Mat4::orthographic_lh(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0),
  );

  while window.update() {
    graphics().clear_color_buffer(Color::WHITE);

    batch.begin(&material);
    batch.draw_sprite(&texture, &SpriteOptions {
      position: Vec2::new(1024.0 / 2.0, 768.0 / 2.0),
      scale: Vec2::new(1.0, 1.0),
      ..Default::default()
    });
    batch.flush();

    window.present()
  }
}
