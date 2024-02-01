//! A backend for Surreal specifically for the GameBoy Advance.

#![no_std]

use common::{ivec2, IVec2};

extern crate alloc;

/// The allocator used by the GameBoy runtime.
///
/// This is a simple bump allocator that allocates from a fixed-size heap.
#[global_allocator]
static HEAP_ALLOCATOR: embedded_alloc::Heap = embedded_alloc::Heap::empty();

/// Initializes the heap allocator used by the runtime.
///
/// The hardware of the GBA is very limited, so we can't use a dynamic
/// allocator. Instead, we use a simple bump allocator that allocates from a
/// fixed-size heap.
fn initialize_heap_allocator() {
  use core::mem::MaybeUninit;

  // allocate the main heap
  const HEAP_SIZE: usize = 4069;
  static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

  unsafe {
    HEAP_ALLOCATOR.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
  }
}

/// The runtime for the GameBoy Advance.
///
/// This is the facade and main entry point to the runtime hardware.
pub struct GameBoyRuntime {}

impl GameBoyRuntime {
  /// Creates a new instance of the GameBoy runtime.
  pub fn new() -> Self {
    Self {}
  }

  /// Entry point for the GameBoy runtime.
  ///
  /// This function is called by the GameBoy runtime when the game is started.
  /// It is responsible for initializing the allocator and then starting the
  /// game.
  ///
  /// The game should never return from this function, and this function needs
  /// to be called prior to using any dynamically allocated memory.
  pub fn start() {
    initialize_heap_allocator();

    loop {
      todo!("run the main loop")
    }
  }
}

/// Represents a color in the GameBoy Advance.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// Represents the audio device of the GameBoy Advance.
pub trait AudioDevice {}

/// Represents the display of the GameBoy Advance.
pub trait DisplayDevice {
  /// Clears the display to the specified color.
  fn clear(&mut self, color: Color);

  /// Draws a pixel to the display.
  fn draw_pixel(&mut self, point: IVec2, color: Color);

  /// Draws a line of pixels to the display.
  fn draw_line(&mut self, from: IVec2, to: IVec2, color: Color) {
    let mut x0 = from.x as i32;
    let mut y0 = from.y as i32;
    let x1 = to.x as i32;
    let y1 = to.y as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    while x0 != x1 || y0 != y1 {
      self.draw_pixel(ivec2(x0, y0), color);

      let e2 = 2 * err;

      if e2 >= dy {
        err += dy;
        x0 += sx;
      }

      if e2 <= dx {
        err += dx;
        y0 += sy;
      }
    }
  }

  /// Draws a rectangle to the display.
  fn draw_rect(&mut self, center: IVec2, width: u32, height: u32, color: Color) {
    let x = center.x as i32 - (width as i32 / 2);
    let y = center.y as i32 - (height as i32 / 2);

    for x in x..x + width as i32 {
      for y in y..y + height as i32 {
        self.draw_pixel(ivec2(x, y), color);
      }
    }
  }

  /// Draws a circle to the display.
  fn draw_circle(&mut self, x: u32, y: u32, radius: u32, color: Color) {
    let mut x0 = radius as i32;
    let mut y0 = 0;
    let mut err = 0;

    while x0 >= y0 {
      self.draw_pixel(ivec2(x as i32 + x0, y as i32 + y0), color);
      self.draw_pixel(ivec2(x as i32 + y0, y as i32 + x0), color);
      self.draw_pixel(ivec2(x as i32 - y0, y as i32 + x0), color);
      self.draw_pixel(ivec2(x as i32 - x0, y as i32 + y0), color);
      self.draw_pixel(ivec2(x as i32 - x0, y as i32 - y0), color);
      self.draw_pixel(ivec2(x as i32 - y0, y as i32 - x0), color);
      self.draw_pixel(ivec2(x as i32 + y0, y as i32 - x0), color);
      self.draw_pixel(ivec2(x as i32 + x0, y as i32 - y0), color);

      y0 += 1;
      err += 1 + 2 * y0;

      if 2 * (err - x0) + 1 > 0 {
        x0 -= 1;
        err += 1 - 2 * x0;
      }
    }
  }

  /// Draws a sprite to the display.
  fn draw_sprite(&mut self, x: i32, y: i32, sprite: &dyn Sprite) {
    for i in 0..sprite.width() {
      for j in 0..sprite.height() {
        let point = ivec2(x + i as i32, y + j as i32);

        self.draw_pixel(point, sprite.sample_pixel(i, j));
      }
    }
  }
}

/// Represents the controller of the GameBoy Advance.

pub trait InputDevice {
  fn is_button_down(&self, button: Button) -> bool;
  fn is_button_up(&self, button: Button) -> bool;
  fn is_button_pressed(&self, button: Button) -> bool;
  fn is_button_released(&self, button: Button) -> bool;
  fn is_button_held(&self, button: Button) -> bool;
  fn is_button_not_held(&self, button: Button) -> bool;
}

/// Represents a sprite that can be drawn to the display.
pub trait Sprite {
  /// The width of the sprite in pixels.
  fn width(&self) -> u32;

  /// The height of the sprite in pixels.
  fn height(&self) -> u32;

  /// Sample a pixel from the sprite
  fn sample_pixel(&self, x: u32, y: u32) -> Color;
}

/// A button on the GameBoy Advance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
  A,
  B,
  Start,
  Select,
  Up,
  Down,
  Left,
  Right,
}
