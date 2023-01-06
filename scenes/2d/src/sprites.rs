use surreal::graphics::*;
use surreal::macros::Object;
use surreal::maths::{EulerRot, Mat4, Vec3Swizzles};
use surreal::scene::*;

/// A [`Component`] which renders a sprite in the game world.
#[derive(Object)]
pub struct SpriteComponent {
  pub region: TextureRegion,
}

impl Component for SpriteComponent {
  fn name(&self) -> &'static str {
    "SpriteComponent"
  }

  fn on_render(&mut self, node: &mut SceneNode, manager: &mut RenderContextManager) {
    manager.with(|context: &mut SpriteContext| {
      context.batch.draw_sprite(
        &self.region,
        &SpriteOptions {
          position: node.local_position().xy(),
          rotation: node.local_rotation().to_euler(EulerRot::XYZ).1,
          scale: node.local_scale().xy(),
          ..Default::default()
        },
      );
    });
  }

  fn kind(&self) -> ComponentKind {
    ComponentKind::Renderer
  }
}

/// A uniform that contains the [`ColorPalette`] texture for sprite rendering.
const UNIFORM_PALETTE: UniformKey<&Texture> = UniformKey::new("u_palette");

/// A uniform that contains the width of the value in [`UNIFORM_PALETTE`].
const UNIFORM_PALETTE_WIDTH: UniformKey<u32> = UniformKey::new("u_paletteWidth");

/// A uniform that contains the projection-view matrix for perspective adjustment.
const UNIFORM_PROJECTION_VIEW: UniformKey<&Mat4> = UniformKey::new("u_projectionView");

/// Represents one of the built-in shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BuiltInShader {
  /// Simple purpose projected sprite shader.
  SpriteStandard,
  /// Palette-shifted sprite shader.
  SpritePalette,
}

// built-in shaders
const SHADER_SPRITE_STANDARD: &str = include_str!("../assets/shaders/sprite-standard.glsl");
const SHADER_SPRITE_PALETTE: &str = include_str!("../assets/shaders/sprite-palette.glsl");

/// Loads a built-in [`ShaderProgram`].
fn load_built_in_shader(graphics: &GraphicsServer, shader: BuiltInShader) -> ShaderProgram {
  let program = match shader {
    BuiltInShader::SpriteStandard => ShaderProgram::from_code(graphics, SHADER_SPRITE_STANDARD),
    BuiltInShader::SpritePalette => ShaderProgram::from_code(graphics, SHADER_SPRITE_PALETTE),
  };

  program.expect(&format!("Failed to load build-in shader {:?}", shader))
}

/// A [`RenderContextDescriptor`] for a simple [`SpriteContext`] for use in sprite rendering.
pub struct SpriteContextDescriptor {
  /// A default projection-view matrix to apply.
  pub projection_view: Mat4,

  /// If a palette is specified, a special shader variant will be loaded that uses the palette.
  /// The palette will be bound to `u_palette` with `u_paletteWidth` texels wide.
  pub palette: Option<ColorPalette<Color>>,

  /// A custom [`ShaderProgram`] to use for rendering.
  pub shader: Option<ShaderProgram>,

  /// The expected number of sprites to use in the batch; used for pre-sizing the batch vertex buffer.
  pub sprite_count: usize,
}

impl Default for SpriteContextDescriptor {
  fn default() -> Self {
    Self {
      projection_view: Mat4::IDENTITY,
      palette: None,
      shader: None,
      sprite_count: 1024,
    }
  }
}

impl RenderContextDescriptor for SpriteContextDescriptor {
  type Context = SpriteContext;

  fn create(&self, graphics: &GraphicsServer) -> Self::Context {
    // determine which shader we're using, prepare material
    let shader = match &self.shader {
      Some(shader) => shader.clone(),
      None => match self.palette {
        // we need a special variant if we're using palette shifting effects
        None => load_built_in_shader(graphics, BuiltInShader::SpriteStandard),
        Some(_) => load_built_in_shader(graphics, BuiltInShader::SpritePalette),
      },
    };

    // prepare the material and sprite batch
    let mut material = Material::new(graphics, &shader);
    let batch = SpriteBatch::with_capacity(graphics, self.sprite_count);

    // prepare the palette texture, if enabled, upload it once
    if let Some(palette) = &self.palette {
      let palette_texture = Texture::new(graphics);

      palette_texture.write_pixels(palette.len(), 1, palette.as_slice());

      material.set_texture(UNIFORM_PALETTE, &palette_texture, None);
      material.set_uniform(UNIFORM_PALETTE_WIDTH, palette.len() as u32);
    }

    // apply the default projection-view matrix
    material.set_uniform(UNIFORM_PROJECTION_VIEW, &self.projection_view);

    // enable default material state
    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    Self::Context { material, batch }
  }
}

/// A simple [`RenderContext`] that allows for sprite rendering using built-in sprite shaders.
#[derive(Object)]
pub struct SpriteContext {
  /// A [`Material`] configured to render sprites.
  pub material: Material,

  /// The [`SpriteBatch`] to use for sprite geometry.
  pub batch: SpriteBatch,
}

impl SpriteContext {
  /// Sets the palette texture on the shader.
  pub fn set_palette(&mut self, palette: &Texture) {
    self.material.set_texture(UNIFORM_PALETTE, palette, None);
  }

  /// Sets the palette width on the shader.
  pub fn set_palette_width(&mut self, width: u32) {
    self.material.set_uniform(UNIFORM_PALETTE_WIDTH, width);
  }

  /// Sets the projection-view matrix on the shader.
  pub fn set_projection_view(&mut self, projection_view: &Mat4) {
    self.material.set_uniform(UNIFORM_PROJECTION_VIEW, projection_view);
  }
}

impl RenderContext for SpriteContext {
  fn on_begin_frame(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.batch.flush();
  }
}

#[cfg(test)]
mod tests {
  use surreal::maths::{vec3, Quat};

  use super::*;

  #[test]
  fn sprite_should_render() {
    let graphics = create_test_graphics();
    let texture = Texture::create_colored(&graphics, 1, 1, Color::RED);

    let graph = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_name("Test")
        .with_local_position(vec3(0., 0., 0.))
        .with_local_rotation(Quat::from_rotation_z(std::f32::consts::PI))
        .with_component(SpriteComponent {
          region: TextureRegion::from(texture),
        }),
    );

    println!("{:?}", graph);
  }
}
