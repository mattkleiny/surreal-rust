//! A simple example of compute shaders in Surreal.

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, Compute!",
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let graphics = &game.host.graphics;

    // build an empty texture, we'll write into it from the GPU.
    let mut texture = Texture::new(graphics);
    texture.write_pixels(1920, 1080, &[Color32::EMPTY; 0]);

    // load our compute shader
    let shader: &ShaderProgram = assets
      .load_asset("assets/shaders/compute-test.glsl")
      .unwrap();

    // bind the texture to the compute shader, write-only access, RGBA8
    shader.set_uniform(
      "u_image",
      &ShaderUniform::ComputeImage(
        texture.clone(),
        0, // texture slot
        ComputeMode::WriteOnly,
        TextureFormat::RGBA8,
      ),
    );

    shader.set_uniform(
      "u_resolution",
      &ShaderUniform::Vector2(vec2(1920.0, 1080.0)),
    );

    // execute the compute shader, wait for it to complete.
    shader.dispatch_compute(1920, 1080, 1);
    graphics.wait_compute_barrier(ComputeBarrier::ImageAccess);

    // prepare a sprite batch so we can re-render the result on screen.
    let mut batch = SpriteBatch::new(graphics);
    let mut material = Material::new(
      graphics,
      &load_built_in_shader(graphics, BuiltInShader::SpriteStandard),
    );

    material.set_uniform("u_texture", &texture);

    game.run_variable_step(|game| {
      game.host.graphics.clear_color_buffer(Color::BLACK);

      // draw what we computed
      batch.begin(&material);
      batch.draw(&TextureRegion::from(&texture), SpriteOptions::default());
      batch.flush();

      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }
    });
  });
}
