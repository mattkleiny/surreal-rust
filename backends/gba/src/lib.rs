//! A backend for Surreal specifically for the GameBoy Advance.

#![no_std]

extern crate alloc;

/// The allocator used by the GameBoy runtime.
///
/// This is a simple bump allocator that allocates from a fixed-size heap.
#[global_allocator]
static HEAP_ALLOCATOR: embedded_alloc::Heap = embedded_alloc::Heap::empty();

/// Entry point for the GameBoy runtime.
///
/// This function is called by the GameBoy runtime when the game is started.
/// It is responsible for initializing the allocator and then starting the game.
///
/// The game should never return from this function.
#[allow(dead_code)]
fn main() -> ! {
  initialize_heap_allocator();

  loop {
    todo!("run the main loop")
  }
}

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

/// Represents the audio device of the GameBoy Advance.
pub trait AudioDevice {}

/// Represents the display of the GameBoy Advance.
pub trait DisplayDevice {
  fn clear(&mut self);
  fn draw_pixel(&mut self, x: u32, y: u32, color: u32);
  fn draw_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: u32);
  fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32);
  fn draw_circle(&mut self, x: u32, y: u32, radius: u32, color: u32);
  fn draw_text(&mut self, x: u32, y: u32, text: &str, color: u32);
  fn draw_text_centered(&mut self, x: u32, y: u32, text: &str, color: u32);
  fn draw_text_right_aligned(&mut self, x: u32, y: u32, text: &str, color: u32);
  fn draw_text_wrapped(&mut self, x: u32, y: u32, text: &str, color: u32);
  fn draw_text_wrapped_centered(&mut self, x: u32, y: u32, text: &str, color: u32);
  fn draw_sprite(&mut self, x: u32, y: u32, sprite: &dyn Sprite);
  fn draw_sprite_centered(&mut self, x: u32, y: u32, sprite: &dyn Sprite);
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
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn pixel(&self, x: u32, y: u32) -> u32;
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
