use super::utils;

#[derive(Debug)]
struct Color {
  r: f64,
  g: f64,
  b: f64,
}
impl Color {
  fn new(r: f64, g: f64, b: f64) -> Self {
    return Color { r, g, b };
  }
}

impl PartialEq<Self> for Color {
  fn eq(&self, other: &Self) -> bool {
    return utils::approx_equals(self.r, other.r)
      && utils::approx_equals(self.g, other.g)
      && utils::approx_equals(self.b, other.b);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color_equality() {
    assert_eq!(Color::new(4., -4., 3.), Color::new(4., -4., 3.));

    assert_eq!(Color::new(1.000000001, 0., 0.), Color::new(1., 0., 0.));
  }
}
