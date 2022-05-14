use surreal::prelude::*;

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  let mut pixels = vec![Color::BLACK; 256 * 256];
  let mut pixels = Grid::new(&mut pixels, 256);

  platform.run(move |platform| unsafe {
    pixels.fill(Color::WHITE);

    let texture = platform.graphics_server.create_texture();

    platform.graphics_server.clear_color_buffer(Color::WHITE);
    platform.graphics_server.write_texture_data(texture, &pixels);

    platform.graphics_server.delete_texture(texture);
  });
}
