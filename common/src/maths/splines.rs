/// A spline is a curve defined by control points.
pub trait Spline {
  /// Returns the value of the spline at the given time.
  fn value(&self, t: f32) -> f32;

  /// Returns the derivative of the spline at the given time.
  fn derivative(&self, t: f32) -> f32;
}

/// A Catmull-Rom [`Spline`].
#[derive(Default, Clone, Debug)]
pub struct CatmulRomSpline {
  pub points: Vec<f32>,
  pub continuous: bool,
}

impl Spline for CatmulRomSpline {
  fn value(&self, t: f32) -> f32 {
    let n = self.points.len() as f32;
    let i = (t * (n - 1.0)).floor() as usize;
    let t = t * (n - 1.0) - i as f32;

    let p0 = if i > 0 {
      self.points[i - 1]
    } else if self.continuous {
      self.points[n as usize - 1]
    } else {
      self.points[0]
    };

    let p1 = self.points[i];
    let p2 = self.points[i + 1];

    let p3 = if i < n as usize - 2 {
      self.points[i + 2]
    } else if self.continuous {
      self.points[0]
    } else {
      self.points[n as usize - 1]
    };

    let t2 = t * t;
    let t3 = t2 * t;

    let m0 = -0.5 * t3 + t2 - 0.5 * t;
    let m1 = 1.5 * t3 - 2.5 * t2 + 1.0;
    let m2 = -1.5 * t3 + 2.0 * t2 + 0.5 * t;
    let m3 = 0.5 * t3 - 0.5 * t2;

    p0 * m0 + p1 * m1 + p2 * m2 + p3 * m3
  }

  fn derivative(&self, t: f32) -> f32 {
    let epsilon = 0.0001; // Small value for finite differences

    let t_plus_epsilon = t + epsilon;
    let t_minus_epsilon = t - epsilon;

    let value_plus_epsilon = self.value(t_plus_epsilon);
    let value_minus_epsilon = self.value(t_minus_epsilon);

    (value_plus_epsilon - value_minus_epsilon) / (2.0 * epsilon)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_catmulrom_spline_value() {
    let spline = CatmulRomSpline {
      points: vec![0.0, 1.0, 2.0, 3.0],
      continuous: false,
    };

    assert_eq!(spline.value(0.0), 0.0);
    assert_eq!(spline.value(0.25), 0.625);
    assert_eq!(spline.value(0.5), 1.0);
    assert_eq!(spline.value(0.75), 2.375);
    assert_eq!(spline.value(1.0), 3.0);
  }

  #[test]
  fn test_catmulrom_spline_derivative() {
    let spline = CatmulRomSpline {
      points: vec![0.0, 1.0, 2.0, 3.0],
      continuous: false,
    };

    assert_eq!(spline.derivative(0.0), 1.0);
    assert_eq!(spline.derivative(0.25), 1.5);
    assert_eq!(spline.derivative(0.5), 1.0);
    assert_eq!(spline.derivative(0.75), 0.5);
    assert_eq!(spline.derivative(1.0), 1.0);
  }
}
