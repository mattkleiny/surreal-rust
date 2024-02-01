//! A backend for Surreal specifically for the GameBoy Advance.

#![no_std] // GBA runs on bare metal, no stdlib

pub struct GameBoy {}

pub trait Display {
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

pub trait Sprite {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn pixel(&self, x: u32, y: u32) -> u32;
}

pub trait Controller {
  fn is_button_down(&self, button: Button) -> bool;
  fn is_button_up(&self, button: Button) -> bool;
  fn is_button_pressed(&self, button: Button) -> bool;
  fn is_button_released(&self, button: Button) -> bool;
  fn is_button_held(&self, button: Button) -> bool;
  fn is_button_not_held(&self, button: Button) -> bool;
}

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
