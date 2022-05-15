use surreal::prelude::*;

#[derive(Default, Debug)]
struct Position(Vector2<f32>);

#[derive(Default, Debug)]
struct Rotation(f32);

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  let mut world = World::new();

  let entity = world.spawn();

  entity.add(Position(vec2(0., 1.)));
  entity.add(Rotation(std::f32::consts::PI * 2.));

  platform.run(|platform| unsafe {
    platform.graphics_server.clear_color_buffer(Color::BLACK);

    world.tick();
  });
}
