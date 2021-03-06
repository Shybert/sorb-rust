use crate::utils::{approx_equals, Lerp};
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

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

  pub fn black() -> Self {
    return Self::new(0., 0., 0.);
  }

  pub fn white() -> Self {
    return Self::new(1., 1., 1.);
  }

  pub fn red() -> Self {
    return Self::new(1., 0., 0.);
  }

  pub fn green() -> Self {
    return Self::new(0., 1., 0.);
  }

  pub fn blue() -> Self {
    return Self::new(0., 0., 1.);
  }

  pub fn yellow() -> Self {
    return Self::new(1., 1., 0.);
  }

  pub fn cyan() -> Self {
    return Self::new(0., 1., 1.);
  }

  pub fn magenta() -> Self {
    return Self::new(1., 0., 1.);
  }

  pub fn set(&mut self, new: &Self) {
    self.r = new.r;
    self.g = new.g;
    self.b = new.b;
  }

  /// Blends two colors by averaging their respective values.
  pub fn blend(&self, other: &Self) -> Self {
    return Color::new(
      (self.r + other.r) / 2.,
      (self.g + other.g) / 2.,
      (self.b + other.b) / 2.,
    );
  }
}

impl Default for Color {
  fn default() -> Self {
    return Self::black();
  }
}
impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    return approx_equals(self.r, other.r)
      && approx_equals(self.g, other.g)
      && approx_equals(self.b, other.b);
  }
}
impl Add for Color {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    return Self::new(self.r + other.r, self.g + other.g, self.b + other.b);
  }
}
impl Sum for Color {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Self>,
  {
    return iter.fold(Color::black(), |final_color, current_color| {
      final_color + current_color
    });
  }
}
impl<'a> Sum<&'a Self> for Color {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Self>,
  {
    return iter.fold(Color::black(), |final_color, &current_color| {
      final_color + current_color
    });
  }
}
impl Sub for Color {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    return Self::new(self.r - other.r, self.g - other.g, self.b - other.b);
  }
}
impl Mul<f64> for Color {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    return Self::new(scalar * self.r, scalar * self.g, scalar * self.b);
  }
}
impl Mul<Color> for f64 {
  type Output = Color;

  fn mul(self, color: Color) -> Color {
    return color * self;
  }
}
impl Mul for Color {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    return Self::new(self.r * other.r, self.g * other.g, self.b * other.b);
  }
}
impl Lerp for Color {
  type Scalar = f64;

  fn lerp(&self, other: &Self, t: &Self::Scalar) -> Self {
    return Color::new(
      self.r.lerp(&other.r, t),
      self.g.lerp(&other.g, t),
      self.b.lerp(&other.b, t),
    );
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
  fn init_black() {
    assert_eq!(Color::black(), Color::new(0., 0., 0.));
  }

  #[test]
  fn init_white() {
    assert_eq!(Color::white(), Color::new(1., 1., 1.));
  }

  #[test]
  fn init_red() {
    assert_eq!(Color::red(), Color::new(1., 0., 0.));
  }

  #[test]
  fn init_green() {
    assert_eq!(Color::green(), Color::new(0., 1., 0.));
  }

  #[test]
  fn init_blue() {
    assert_eq!(Color::blue(), Color::new(0., 0., 1.));
  }

  #[test]
  fn init_yellow() {
    assert_eq!(Color::yellow(), Color::new(1., 1., 0.));
  }

  #[test]
  fn init_cyan() {
    assert_eq!(Color::cyan(), Color::new(0., 1., 1.));
  }

  #[test]
  fn init_magenta() {
    assert_eq!(Color::magenta(), Color::new(1., 0., 1.));
  }

  #[test]
  fn init_default() {
    assert_eq!(Color::default(), Color::black());
  }

  #[test]
  fn equality() {
    assert_eq!(Color::new(4., -4., 3.), Color::new(4., -4., 3.));

    assert_eq!(Color::new(1.000000001, 0., 0.), Color::new(1., 0., 0.));
  }

  #[test]
  fn addition() {
    assert_eq!(
      Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25),
      Color::new(1.6, 0.7, 1.)
    );
  }

  #[test]
  fn sum() {
    let colors = vec![
      Color::new(0.1, 0.1, 0.1),
      Color::new(0.2, 0.2, 0.2),
      Color::new(0.3, 0.3, 0.3),
    ];
    let sum: Color = colors.into_iter().sum();
    assert_eq!(sum, Color::new(0.6, 0.6, 0.6));
  }
  #[test]
  fn sum_references() {
    let colors = vec![
      Color::new(0.1, 0.1, 0.1),
      Color::new(0.2, 0.2, 0.2),
      Color::new(0.3, 0.3, 0.3),
    ];
    let sum: Color = colors.iter().sum();
    assert_eq!(sum, Color::new(0.6, 0.6, 0.6));
  }

  #[test]
  fn subtraction() {
    assert_eq!(
      Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25),
      Color::new(0.2, 0.5, 0.5)
    );
  }

  #[test]
  fn scalar_multiplication() {
    assert_eq!(Color::new(0.2, 0.3, 0.4) * 2., Color::new(0.4, 0.6, 0.8));
    assert_eq!(2. * Color::new(0.2, 0.3, 0.4), Color::new(0.4, 0.6, 0.8));

    assert_eq!(0.5 * Color::new(0.2, 0.3, 0.4), Color::new(0.1, 0.15, 0.2));
  }

  #[test]
  fn color_multiplication() {
    assert_eq!(
      Color::new(1., 0.2, 0.4) * Color::new(0.9, 1., 0.1),
      Color::new(0.9, 0.2, 0.04)
    );
  }

  #[test]
  fn set() {
    let mut old = Color::default();
    let new = Color::new(-0.3, 0.45, 1.);
    old.set(&new);
    assert_eq!(old, new);
  }

  #[test]
  fn blend() {
    assert_eq!(Color::red().blend(&Color::blue()), Color::new(0.5, 0., 0.5));
    assert_eq!(
      Color::new(-0.4, 0.7, 0.3).blend(&Color::new(0.6, 0.3, 0.1)),
      Color::new(0.1, 0.5, 0.2)
    );
    assert_eq!(
      Color::new(-1.2, -0.3, -0.5).blend(&Color::new(-0.6, -0.1, -0.7)),
      Color::new(-0.9, -0.2, -0.6)
    );
  }

  #[test]
  fn lerp() {
    assert_eq!(
      Color::black().lerp(&Color::white(), &0.5),
      Color::new(0.5, 0.5, 0.5)
    );
    assert_eq!(Color::red().lerp(&Color::blue(), &1.), Color::blue());
    assert_eq!(Color::red().lerp(&Color::blue(), &0.), Color::red());
    assert_eq!(
      Color::new(0.6, 0.3, 0.8).lerp(&Color::new(0.9, 0.2, 0.5), &0.75),
      Color::new(0.825, 0.225, 0.575)
    );
  }
}
