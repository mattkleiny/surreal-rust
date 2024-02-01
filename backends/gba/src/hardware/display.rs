//! Display support for the GameBoy Advance.

use crate::GameBoyRuntime;

pub struct IVec2 {
  pub x: i32,
  pub y: i32,
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
  IVec2 { x, y }
}

/// Represents a sprite that can be drawn to the display.
pub trait Sprite {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn sample_pixel(&self, x: u32, y: u32) -> Color;
}

/// Represents a color in the GameBoy Advance.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// Represents the display of the GameBoy Advance.
pub trait DisplayDevice {
  fn clear(&mut self, color: Color);
  fn draw_pixel(&mut self, point: IVec2, color: Color);

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

  fn draw_rect(&mut self, center: IVec2, width: u32, height: u32, color: Color) {
    let x = center.x as i32 - (width as i32 / 2);
    let y = center.y as i32 - (height as i32 / 2);

    for x in x..x + width as i32 {
      for y in y..y + height as i32 {
        self.draw_pixel(ivec2(x, y), color);
      }
    }
  }

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

  fn draw_sprite(&mut self, x: i32, y: i32, sprite: &dyn Sprite) {
    for i in 0..sprite.width() {
      for j in 0..sprite.height() {
        let point = ivec2(x + i as i32, y + j as i32);

        self.draw_pixel(point, sprite.sample_pixel(i, j));
      }
    }
  }
}

impl DisplayDevice for GameBoyRuntime {
  fn clear(&mut self, color: Color) {
    todo!()
  }

  fn draw_pixel(&mut self, point: IVec2, color: Color) {
    todo!()
  }
}
