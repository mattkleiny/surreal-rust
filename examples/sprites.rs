use surreal::{backends::sdl::*, common::*, graphics::*};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Sprite Example".to_string(),
    ..Default::default()
  })
  .expect("Failed to create window");

  let graphics = GraphicsEngine::opengl(&window);
  let mut batch = SpriteBatch::new(&graphics).unwrap();
  let mut material = Material::from_template(&graphics, &SHADER_SPRITE_STANDARD).unwrap();
  let texture = Texture::from_path(&graphics, "assets/sprites/bunny.png")
    .unwrap()
    .to_region();

  material.set_blend_state(BlendState::Enabled {
    source: BlendFactor::SrcAlpha,
    destination: BlendFactor::OneMinusSrcAlpha,
  });

  material.set_uniform(
    PROJECTION_VIEW,
    &Mat4::orthographic_lh(0.0, 800.0, 600.0, 0.0, -1.0, 1.0),
  );

  while window.update() {
    graphics.clear_color_buffer(Color::BLACK);

    batch.begin(&material);

    batch.draw_sprite(&texture, &SpriteOptions {
      position: vec2(400.0, 300.0),
      ..Default::default()
    });

    batch.flush();

    window.present();
  }
}
