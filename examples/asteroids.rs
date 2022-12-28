//! An asteroids clone in Rust with manual rasterization into a pixel grid.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Asteroids",
    log_level: LevelFilter::Trace,
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let mut canvas = PixelCanvas::<Color32>::new(&engine.graphics, 256, 144);
    let polygon = Polygon::triangle(4.);

    let mut position = vec2(64., 64.);
    let mut rotation = 0.;

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      canvas.pixels.clear();
      canvas.pixels.draw(Color32::WHITE, &polygon.rotate(rotation).translate(position));

      canvas.draw();

      if let Some(keyboard) = &engine.input.keyboard {
        let forward = vec2(0., 1.).rotate(rotation);

        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }

        if keyboard.is_key_down(Key::W) {
          position -= forward * tick.time.delta_time * 10.;
        }

        if keyboard.is_key_down(Key::S) {
          position += forward * tick.time.delta_time * 10.;
        }

        if keyboard.is_key_down(Key::Q) {
          rotation -= tick.time.delta_time * 10.;
        }

        if keyboard.is_key_down(Key::E) {
          rotation += tick.time.delta_time * 10.;
        }
      }
    });
  });
}

/// A polygon shape that can be rasterized to a `Grid`.
struct Polygon {
  vertices: Vec<Vector2<f32>>,
}

impl Polygon {
  #[rustfmt::skip]
  pub fn triangle(size: f32) -> Self {
    Self {
      vertices: vec![
        vec2(-size, size),
        vec2(0., -size),
        vec2(size, size),
      ],
    }
  }

  /// Computes the rectilinear bounds of the polygon.
  pub fn bounds(&self) -> Rectangle<f32> {
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = 0.;
    let mut max_y = 0.;

    for vertex in &self.vertices {
      if vertex.x < min_x {
        min_x = vertex.x
      }
      if vertex.y < min_y {
        min_y = vertex.y
      }
      if vertex.x > max_x {
        max_x = vertex.x
      }
      if vertex.y > max_x {
        max_y = vertex.y
      }
    }

    return Rectangle::from_corner_points(min_x, min_y, max_x, max_y);
  }

  /// Translates the polygon by the given vector.
  pub fn translate(&self, offset: Vector2<f32>) -> Self {
    Self {
      vertices: self.vertices.iter().map(|v| *v + offset).collect(),
    }
  }

  /// Rotates the polygon by the given angle (in radians).
  pub fn rotate(&self, angle: f32) -> Self {
    Self {
      vertices: self.vertices.iter().map(|v| v.rotate(angle)).collect(),
    }
  }

  /// Determines if the polygon contains the given point.
  pub fn contains(&self, point: Vector2<f32>) -> bool {
    #[inline]
    fn is_ccw(a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>) -> bool {
      (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }

    #[inline]
    fn intersects(a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>, d: Vector2<f32>) -> bool {
      is_ccw(a, c, d) != is_ccw(b, c, d) && is_ccw(a, b, c) != is_ccw(a, b, d)
    }

    let direction = Vector2 { x: 1000., ..point };
    let mut count = 0;

    for i in 0..self.vertices.len() {
      let start = self.vertices[i];
      let end = if i == self.vertices.len() - 1 {
        self.vertices[0]
      } else {
        self.vertices[i + 1]
      };

      if intersects(point, direction, start, end) {
        count += 1;
      }
    }

    count % 2 == 1
  }
}

impl Shape for Polygon {
  fn rasterize<T: Clone>(&self, value: T, target: &mut Grid<T>) {
    let bounds = self.bounds();

    for y in bounds.top() as i32..bounds.bottom() as i32 {
      for x in bounds.left() as i32..bounds.right() as i32 {
        let point = vec2(x as f32, y as f32);

        if self.contains(point) {
          target.set(x, y, value.clone());
        }
      }
    }
  }
}
