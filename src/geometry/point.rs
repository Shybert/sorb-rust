use crate::geometry::Vector;
use crate::utils::approx_equals;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    return Self { x, y, z };
  }

  pub fn origin() -> Self {
    return Self::new(0., 0., 0.);
  }
}

impl Default for Point {
  fn default() -> Self {
    return Self::origin();
  }
}
impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    return approx_equals(self.x, other.x)
      && approx_equals(self.y, other.y)
      && approx_equals(self.z, other.z);
  }
}
impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    return Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
  }
}
impl Add<Vector> for Point {
  type Output = Self;

  fn add(self, vector: Vector) -> Self {
    return Self::new(self.x + vector.x, self.y + vector.y, self.z + vector.z);
  }
}
impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    return Self::new(self.x - other.x, self.y - other.y, self.z - other.z);
  }
}
impl Neg for Point {
  type Output = Self;

  fn neg(self) -> Self {
    return Self::new(-self.x, -self.y, -self.z);
  }
}
impl Mul<f64> for Point {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    return Self::new(scalar * self.x, scalar * self.y, scalar * self.z);
  }
}
impl Mul<Point> for f64 {
  type Output = Point;

  fn mul(self, point: Point) -> Point {
    return point * self;
  }
}
impl Div<f64> for Point {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    return Self::new(self.x / scalar, self.y / scalar, self.z / scalar);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let point = Point::new(1., 2., 3.);
    assert_eq!(point.x, 1.);
    assert_eq!(point.y, 2.);
    assert_eq!(point.z, 3.);
  }

  #[test]
  fn init_origin() {
    assert_eq!(Point::origin(), Point::new(0., 0., 0.,));
  }

  #[test]
  fn init_default() {
    assert_eq!(Point::default(), Point::origin());
  }

  #[test]
  fn equality() {
    assert_eq!(Point::new(4., -4., 3.), Point::new(4., -4., 3.));

    assert_eq!(Point::new(1.000000001, 0., 0.), Point::new(1., 0., 0.));
  }

  #[test]
  fn addition_point() {
    assert_eq!(
      Point::new(3., -2., 5.) + Point::new(-2., 3., 1.),
      Point::new(1., 1., 6.)
    )
  }

  #[test]
  fn addition_vector() {
    assert_eq!(
      Point::new(3., -2., 5.) + Vector::new(-2., 3., 1.),
      Point::new(1., 1., 6.)
    );
  }

  #[test]
  fn subtraction() {
    assert_eq!(
      Point::new(3., 2., 1.) - Point::new(5., 6., 7.),
      Point::new(-2., -4., -6.)
    )
  }

  #[test]
  fn negation() {
    assert_eq!(-Point::new(1., -2., 3.), Point::new(-1., 2., -3.))
  }

  #[test]
  fn scalar_multiplication() {
    assert_eq!(Point::new(1., -2., 3.) * 3.5, Point::new(3.5, -7., 10.5));
    assert_eq!(3.5 * Point::new(1., -2., 3.), Point::new(3.5, -7., 10.5));

    assert_eq!(0.5 * Point::new(1., -2., 3.), Point::new(0.5, -1., 1.5));
  }

  #[test]
  fn scalar_division() {
    assert_eq!(Point::new(1., -2., 3.) / 2., Point::new(0.5, -1., 1.5));
  }
}
