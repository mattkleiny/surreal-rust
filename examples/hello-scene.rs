use surreal::{backends::sdl::*, prelude::*};

fn main() -> surreal::common::Result<()> {
  let window = Window::new(&WindowSettings {
    title: "Hello Scene!",
    ..Default::default()
  })?;

  let graphics = GraphicsEngine::opengl(&window);

  let mut scene = SceneGraph2D::default();
  let mut pipeline = MultiPassPipeline::new_forward_pipeline(&graphics);
  let mut clock = DeltaClock::default();

  while window.update() {
    let delta_time = clock.tick();

    scene.update(delta_time);
    pipeline.render(&scene);

    window.present();
  }

  Ok(())
}
