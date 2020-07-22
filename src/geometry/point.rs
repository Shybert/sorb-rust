use crate::utils::approx_equals;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
impl Point {
  fn new() -> Self {
    return Point {
      x: 0.,
      y: 0.,
      z: 0.,
    };
  }

  pub fn from(x: f64, y: f64, z: f64) -> Self {
    return Point { x, y, z };
  }
}

impl PartialEq<Self> for Point {
  fn eq(&self, other: &Self) -> bool {
    return approx_equals(&self.x, &other.x)
      && approx_equals(&self.y, &other.y)
      && approx_equals(&self.z, &other.z);
  }
}
impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    return Self::from(self.x + other.x, self.y + other.y, self.z + other.z);
  }
}
impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    return Self::from(self.x - other.x, self.y - other.y, self.z - other.z);
  }
}
impl Neg for Point {
  type Output = Self;

  fn neg(self) -> Self {
    return Self::from(-self.x, -self.y, -self.z);
  }
}
impl Mul<f64> for Point {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    return Self::from(scalar * self.x, scalar * self.y, scalar * self.z);
  }
}
impl Mul<Point> for f64 {
  type Output = Point;

  fn mul(self, point: Point) -> Point {
    return Point::from(self * point.x, self * point.y, self * point.z);
  }
}
impl Div<f64> for Point {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    return Self::from(self.x / scalar, self.y / scalar, self.z / scalar);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let point = Point::new();
    assert_eq!(point.x, 0.);
    assert_eq!(point.y, 0.);
    assert_eq!(point.z, 0.);
  }

  #[test]
  fn init_from() {
    let point = Point::from(1., 2., 3.);
    assert_eq!(point.x, 1.);
    assert_eq!(point.y, 2.);
    assert_eq!(point.z, 3.);
  }

  #[test]
  fn equality() {
    assert_eq!(Point::from(4., -4., 3.), Point::from(4., -4., 3.));

    assert_eq!(Point::from(1.000000001, 0., 0.), Point::from(1., 0., 0.));
  }

  #[test]
  fn addition() {
    assert_eq!(
      Point::from(3., -2., 5.) + Point::from(-2., 3., 1.),
      Point::from(1., 1., 6.)
    )
  }

  #[test]
  fn subtraction() {
    assert_eq!(
      Point::from(3., 2., 1.) - Point::from(5., 6., 7.),
      Point::from(-2., -4., -6.)
    )
  }

  #[test]
  fn negation() {
    assert_eq!(-Point::from(1., -2., 3.), Point::from(-1., 2., -3.))
  }

  #[test]
  fn scalar_multiplication() {
    assert_eq!(Point::from(1., -2., 3.) * 3.5, Point::from(3.5, -7., 10.5));
    assert_eq!(3.5 * Point::from(1., -2., 3.), Point::from(3.5, -7., 10.5));

    assert_eq!(0.5 * Point::from(1., -2., 3.), Point::from(0.5, -1., 1.5));
  }

  #[test]
  fn scalar_division() {
    assert_eq!(Point::from(1., -2., 3.) / 2., Point::from(0.5, -1., 1.5));
  }
}
