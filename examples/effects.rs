//! An example of effect/render targets for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Effects",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let graphics = &game.host.graphics;

    // TODO: asset management
    let mut texture = Texture::new(graphics);
    let image = Image::from_path("assets/sprites/bunny.png", None).expect("Failed to load sprite image");
    texture.write_image(&image);
    let sprite = TextureRegion::from(&texture); // TODO: simplify this

    let mut sprite_material = Material::new(graphics, &load_standard_shader(graphics, BuiltInShader::Sprite(BuiltInSpriteShader::Standard)));
    let mut effect_material = Material::new(graphics, &load_standard_shader(graphics, BuiltInShader::Effect(BuiltInEffect::Aberration)));

    let mut batch = SpriteBatch::new(graphics);

    let render_target = RenderTarget::new(graphics, &RenderTargetDescriptor {
      color_attachment: RenderTextureDescriptor {
        width: 1280,
        height: 720,
        options: TextureOptions::default(),
      },
      depth_attachment: None,
      stencil_attachment: None,
    });

    // set-up camera perspective
    let projection_view = Matrix4x4::create_orthographic(1280., 720., 0., 100.);

    effect_material.set_uniform("u_projectionView", &projection_view);
    sprite_material.set_uniform("u_projectionView", &projection_view);
    sprite_material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha
    });

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(Color::BLACK);

      // TODO: simplify this API
      render_target.activate();
      {
        batch.begin(&sprite_material);
        batch.draw(&sprite, SpriteOptions::default());
        batch.flush();
      }
      render_target.deactivate();

      // interpolate intensity over time
      {
        let intensity = (context.time.total_time.sin() + 1. / 2.) * 0.005;
        let region = TextureRegion::from(&render_target.color_attachment());

        effect_material.set_uniform("u_intensity", intensity);

        batch.begin(&effect_material);
        batch.draw(&region, SpriteOptions::default());
        batch.flush();
      }

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
