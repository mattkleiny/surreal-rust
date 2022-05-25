//! A simple set of tools for rapid prototyping.

pub use pixels::*;

use crate::graphics::{ColorPalette, GraphicsServer, Pixel, ShaderProgram};

mod pixels;

// TODO: make this easier to use?

const STANDARD_SHADER: &'static str = include_str!("../assets/shaders/standard.glsl");

const PALETTE_AYY_4: &'static [u8] = include_bytes!("../assets/palettes/ayy-4.pal");
const PALETTE_DEMICHROME_4: &'static [u8] = include_bytes!("../assets/palettes/demichrome-4.pal");
const PALETTE_HOLLOW_4: &'static [u8] = include_bytes!("../assets/palettes/hollow-4.pal");
const PALETTE_KULE_16: &'static [u8] = include_bytes!("../assets/palettes/kule-16.pal");
const PALETTE_LOW_8: &'static [u8] = include_bytes!("../assets/palettes/low-8.pal");
const PALETTE_SPACE_DUST_9: &'static [u8] = include_bytes!("../assets/palettes/space-dust-9.pal");

/// Represents one of the embedded color palettes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInPalette {
  Ayy4,
  Demichrome4,
  Hollow4,
  Kule16,
  Low8,
  SpaceDust9,
}

/// Loads the standard shader program from embedded resources.
pub fn load_standard_shader(server: &GraphicsServer) -> ShaderProgram {
  ShaderProgram::from_string(server, STANDARD_SHADER).expect("Failed to load standard shader")
}

/// Loads the given built-in color palette.
pub fn load_standard_palette<P>(palette: BuiltInPalette) -> ColorPalette<P> where P: Pixel {
  match palette {
    BuiltInPalette::Ayy4 => ColorPalette::from_bytes(PALETTE_AYY_4).expect("Failed to load standard palette"),
    BuiltInPalette::Demichrome4 => ColorPalette::from_bytes(PALETTE_DEMICHROME_4).expect("Failed to load standard palette"),
    BuiltInPalette::Hollow4 => ColorPalette::from_bytes(PALETTE_HOLLOW_4).expect("Failed to load standard palette"),
    BuiltInPalette::Kule16 => ColorPalette::from_bytes(PALETTE_KULE_16).expect("Failed to load standard palette"),
    BuiltInPalette::Low8 => ColorPalette::from_bytes(PALETTE_LOW_8).expect("Failed to load standard palette"),
    BuiltInPalette::SpaceDust9 => ColorPalette::from_bytes(PALETTE_SPACE_DUST_9).expect("Failed to load standard palette"),
  }
}