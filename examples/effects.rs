//! An example of effect/render targets for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Effects",
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let graphics = &game.host.graphics;

    // set-up rendering
    let sprite: &Texture = assets.load_asset("assets/sprites/bunny.png").expect("Failed to load sprite image");
    let region = TextureRegion::from(sprite);

    let mut sprite_material = Material::new(graphics, &load_built_in_shader(graphics, BuiltInShader::SpriteStandard));
    let mut effect_material = Material::new(graphics, &load_built_in_shader(graphics, BuiltInShader::AberrationEffect));

    let render_target = RenderTarget::new(
      graphics,
      &RenderTargetDescriptor {
        color_attachment: RenderTextureDescriptor {
          width: 1280,
          height: 720,
          options: TextureOptions::default(),
        },
        depth_attachment: None,
        stencil_attachment: None,
      },
    );

    let mut batch = SpriteBatch::new(graphics);

    let projection_view = Matrix4x4::create_orthographic(1280., 720., 0., 100.);

    effect_material.set_uniform("u_projectionView", &projection_view);
    sprite_material.set_uniform("u_projectionView", &projection_view);

    sprite_material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    game.run_variable_step(|game| {
      game.host.graphics.clear_color_buffer(Color::BLACK);

      // TODO: simplify this API (draw list or something?)
      // draw the main display
      {
        render_target.activate();

        batch.begin(&sprite_material);
        batch.draw(&region, SpriteOptions::default());
        batch.flush();

        render_target.deactivate();
      }

      // render the effect
      {
        // interpolate intensity over time
        let intensity = (game.time.total_time.sin() + 1. / 2.) * 0.005;
        let color_attachment = render_target.color_attachment();
        let region = TextureRegion::from(&color_attachment);

        effect_material.set_uniform("u_intensity", intensity);

        batch.begin(&effect_material);
        batch.draw(&region, SpriteOptions::default());
        batch.flush();
      }

      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }
    });
  });
}
