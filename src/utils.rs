/// Constant used to accommodate for floating-point rounding error.
pub const EPSILON: f64 = 0.00001;
pub fn approx_equals(a: f64, b: f64) -> bool {
  return (a - b).abs() < EPSILON;
}
#[macro_export]
macro_rules! assert_ae {
  ($left:expr, $right:expr) => {
    if !approx_equals($left, $right) {
      panic!(
        "assertion failed: `(left == right)`
left:  `{}`
right: `{}`",
        $left, $right
      )
    }
  };
}

pub fn clamp_number(number: f64, min: f64, max: f64) -> f64 {
  return if number < min {
    min
  } else if number > max {
    max
  } else {
    number
  };
}

pub fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
  let discriminant = b.powi(2) - 4. * a * c;
  if discriminant < 0. {
    return None;
  }

  let (mut x1, mut x2) = (
    (-b - discriminant.sqrt()) / (2. * a),
    (-b + discriminant.sqrt()) / (2. * a),
  );
  if x1 > x2 {
    std::mem::swap(&mut x1, &mut x2);
  };
  return Some((x1, x2));
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ae;

  #[test]
  fn approx_equals_difference_less_than_epsilon() {
    assert_eq!(approx_equals(0.000000001, 0.0000000001), true);
  }

  #[test]
  fn approx_equals_difference_larger_than_epsilon() {
    assert_eq!(approx_equals(0.00001, 0.00002), false);
  }

  #[test]
  fn clamp_number_above_max() {
    assert_eq!(clamp_number(256., 0., 255.), 255.);
  }

  #[test]
  fn clamp_number_at_max() {
    assert_eq!(clamp_number(255., 0., 255.), 255.);
  }

  #[test]
  fn clamp_number_below_min() {
    assert_eq!(clamp_number(-1., 0., 255.), 0.);
  }

  #[test]
  fn clamp_number_at_min() {
    assert_eq!(clamp_number(0., 0., 255.), 0.);
  }

  #[test]
  fn clamp_number_between() {
    assert_eq!(clamp_number(128., 0., 255.), 128.);
  }

  #[test]
  fn quadratic_two_roots() {
    let roots = quadratic(2., -5., 0.);
    let (x1, x2) = roots.expect("Expected roots");
    assert_ae!(x1, 0.);
    assert_ae!(x2, 2.5);
  }

  #[test]
  fn quadratic_single_root() {
    let roots = quadratic(2., 4., 2.);
    let (x1, x2) = roots.expect("Expected roots");
    assert_ae!(x1, -1.);
    assert_ae!(x2, -1.);
  }

  #[test]
  fn quadratic_zero_roots() {
    let roots = quadratic(2., 4., 9.);
    assert!(roots.is_none());
  }

  #[test]
  fn quadratic_large_floating_point_numbers() {
    let roots = quadratic(-299.45, 392348.998, 889.000987);
    let (x1, x2) = roots.expect("Expected roots");
    assert_ae!(x1, -0.00227);
    assert_ae!(x2, 1310.23435);
  }
}
