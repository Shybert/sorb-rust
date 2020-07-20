pub fn approx_equals(a: &f64, b: &f64) -> bool {
  return (a - b).abs() < 0.00001;
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

#[cfg(test)]
mod tests {
  use super::*;

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
}
