use crate::utils::approx_equals;

#[derive(Clone, Copy, Debug)]
pub struct Color {
  pub r: f64,
  pub g: f64,
  pub b: f64,
}
impl Color {
  pub fn new(r: f64, g: f64, b: f64) -> Self {
    return Self { r, g, b };
  }

  pub fn set(&mut self, new: &Self) {
    self.r = new.r;
    self.g = new.g;
    self.b = new.b;
  }
}

impl Default for Color {
  fn default() -> Self {
    return Self::new(0., 0., 0.);
  }
}
impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    return approx_equals(self.r, other.r)
      && approx_equals(self.g, other.g)
      && approx_equals(self.b, other.b);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ae;

  #[test]
  fn init_new() {
    let color = Color::new(0.5, 0.7, 0.12443);
    assert_ae!(color.r, 0.5);
    assert_ae!(color.g, 0.7);
    assert_ae!(color.b, 0.12443);
  }

  #[test]
  fn init_default() {
    let black = Color::new(0., 0., 0.);
    assert_eq!(Color::default(), black);
  }

  #[test]
  fn equality() {
    assert_eq!(Color::new(4., -4., 3.), Color::new(4., -4., 3.));

    assert_eq!(Color::new(1.000000001, 0., 0.), Color::new(1., 0., 0.));
  }

  #[test]
  fn set() {
    let mut old = Color::new(0., 0., 0.);
    let new = Color::new(-0.3, 0.45, 1.);
    old.set(&new);
    assert_eq!(old, new);
  }
}
