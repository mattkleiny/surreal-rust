use super::*;

/// Represents a curve on a plane in 2-space.
pub trait Curve {
  fn evaluate(&self, t: f32) -> Vec2;
}

/// A linear curve in 2-space.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
  pub a: Vec2,
  pub b: Vec2,
}

impl Curve for Line {
  fn evaluate(&self, t: f32) -> Vec2 {
    self.a.lerp(self.b, t)
  }
}

/// Represents a quadratic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadraticBezier {
  pub start: Vec2,
  pub control: Vec2,
  pub end: Vec2,
}

impl Curve for QuadraticBezier {
  fn evaluate(&self, t: f32) -> Vec2 {
    let a = self.start.lerp(self.control, t);
    let b = self.control.lerp(self.end, t);

    a.lerp(b, t)
  }
}

/// Represents a cubic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CubicBezier {
  pub start: Vec2,
  pub control1: Vec2,
  pub control2: Vec2,
  pub end: Vec2,
}

impl Curve for CubicBezier {
  fn evaluate(&self, t: f32) -> Vec2 {
    let a = self.start.lerp(self.control1, t);
    let b = self.control1.lerp(self.control2, t);
    let c = self.control2.lerp(self.end, t);

    let d = a.lerp(b, t);
    let e = b.lerp(c, t);

    d.lerp(e, t)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_line_evaluate() {
    let curve = Line {
      a: Vec2 { x: 0.0, y: 0.0 },
      b: Vec2 { x: 2.0, y: 2.0 },
    };

    assert_eq!(curve.evaluate(0.0), vec2(0.0, 0.0));
    assert_eq!(curve.evaluate(0.5), vec2(1.0, 1.0));
    assert_eq!(curve.evaluate(1.0), vec2(2.0, 2.0));
  }

  #[test]
  fn test_quadratic_bezier_evaluate() {
    let curve = QuadraticBezier {
      start: Vec2 { x: 0.0, y: 0.0 },
      control: Vec2 { x: 1.0, y: 2.0 },
      end: Vec2 { x: 2.0, y: 0.0 },
    };

    assert_eq!(curve.evaluate(0.0), vec2(0.0, 0.0));
    assert_eq!(curve.evaluate(0.5), vec2(1.0, 1.0));
    assert_eq!(curve.evaluate(1.0), vec2(2.0, 0.0));
  }

  #[test]
  fn test_cubic_bezier_evaluate() {
    let curve = CubicBezier {
      start: Vec2 { x: 0.0, y: 0.0 },
      control1: Vec2 { x: 1.0, y: 2.0 },
      control2: Vec2 { x: 3.0, y: 4.0 },
      end: Vec2 { x: 4.0, y: 0.0 },
    };

    assert_eq!(curve.evaluate(0.0), vec2(0.0, 0.0));
    assert_eq!(curve.evaluate(0.5), vec2(2.0, 2.25));
    assert_eq!(curve.evaluate(1.0), vec2(4.0, 0.0));
  }
}
