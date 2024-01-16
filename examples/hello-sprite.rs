use core::maths::Mat4;

use graphics::{Color, GraphicsEngine, Material, ShaderProgram, SpriteBatch, Texture, Window, WindowSettings};

fn main() -> surreal::core::Result<()> {
  let window = Window::new(&WindowSettings {
    title: "Hello Sprite!",
    ..Default::default()
  })?;

  let graphics = GraphicsEngine::create_opengl(&window);

  let mut batch = SpriteBatch::new(&graphics)?;
  let shader = ShaderProgram::from_glsl(&graphics, include_str!("../assets/shaders/canvas-standard.glsl"))?;
  let mut material = Material::new(&graphics, &shader);
  let texture = Texture::from_color(&graphics, 1, 1, Color::RED)?;

  while window.update() {
    graphics.clear_color_buffer(Color::BLACK);

    material.set_uniform("u_projectionView", Mat4::IDENTITY);
    material.set_texture("u_texture", &texture, None);

    batch.begin(&material);
    batch.flush();

    window.present();
  }

  Ok(())
}
