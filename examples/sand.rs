#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let palette = ColorPalette::from_jasc_file("assets/palettes/hollow-4.pal").expect("Failed to load color palette");
    let shader = ShaderProgram::load(&game.host.graphics, "assets/shaders/standard.glsl").expect("Failed to load shader program");
    let mut canvas = Grid::new(512, 512);
    let mut material = Material::new(&game.host.graphics, &shader);
    let mut texture = Texture::new(&game.host.graphics);
    let mesh = Mesh::create_quad(&game.host.graphics, 1.);

    material.set_uniform("u_projectionView", Matrix4x4::IDENTITY);
    material.set_texture("u_texture", texture.handle(), 0, None);

    canvas.fill(Color::BLACK);

    game.run_variable_step(|context| unsafe {
      context.host.graphics.clear_color_buffer(palette[0]);

      texture.write_pixels(canvas.width(), canvas.height(), &canvas.as_slice());

      mesh.draw(&material, PrimitiveTopology::Triangles);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}