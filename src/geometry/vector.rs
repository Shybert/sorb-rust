use crate::utils;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
struct Vector {
  x: f64,
  y: f64,
  z: f64,
}
impl Vector {
  fn new(x: f64, y: f64, z: f64) -> Self {
    return Vector { x, y, z };
  }

  fn magnitude(&self) -> f64 {
    return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
  }

  fn normalize(&self) -> Self {
    let magnitude = self.magnitude();
    return Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude);
  }
}

impl PartialEq<Self> for Vector {
  fn eq(&self, other: &Self) -> bool {
    return utils::approx_equals(self.x, other.x)
      && utils::approx_equals(self.y, other.y)
      && utils::approx_equals(self.z, other.z);
  }
}
impl Add for Vector {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    return Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
  }
}
impl Sub for Vector {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    return Self::new(self.x - other.x, self.y - other.y, self.z - other.z);
  }
}
impl Neg for Vector {
  type Output = Self;

  fn neg(self) -> Self {
    return Self::new(-self.x, -self.y, -self.z);
  }
}
impl Mul<f64> for Vector {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    return Self::new(scalar * self.x, scalar * self.y, scalar * self.z);
  }
}
impl Mul<Vector> for f64 {
  type Output = Vector;

  fn mul(self, vector: Vector) -> Vector {
    return Vector::new(self * vector.x, self * vector.y, self * vector.z);
  }
}
impl Div<f64> for Vector {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    return Self::new(self.x / scalar, self.y / scalar, self.z / scalar);
  }
}

fn dot(a: &Vector, b: &Vector) -> f64 {
  return a.x * b.x + a.y * b.y + a.z * b.z;
}

fn cross(a: &Vector, b: &Vector) -> Vector {
  return Vector::new(
    a.y * b.z - a.z * b.y,
    a.z * b.x - a.x * b.z,
    a.x * b.y - a.y * b.x,
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn equality() {
    assert_eq!(Vector::new(4., -4., 3.), Vector::new(4., -4., 3.));

    assert_eq!(Vector::new(1.000000001, 0., 0.), Vector::new(1., 0., 0.));
  }

  #[test]
  fn addition() {
    assert_eq!(
      Vector::new(3., -2., 5.) + Vector::new(-2., 3., 1.),
      Vector::new(1., 1., 6.)
    )
  }

  #[test]
  fn subtraction() {
    assert_eq!(
      Vector::new(3., 2., 1.) - Vector::new(5., 6., 7.),
      Vector::new(-2., -4., -6.)
    )
  }

  #[test]
  fn negation() {
    assert_eq!(-Vector::new(1., -2., 3.), Vector::new(-1., 2., -3.))
  }

  #[test]
  fn scalar_multiplication() {
    assert_eq!(Vector::new(1., -2., 3.) * 3.5, Vector::new(3.5, -7., 10.5));
    assert_eq!(3.5 * Vector::new(1., -2., 3.), Vector::new(3.5, -7., 10.5));

    assert_eq!(0.5 * Vector::new(1., -2., 3.), Vector::new(0.5, -1., 1.5));
  }

  #[test]
  fn scalar_division() {
    assert_eq!(Vector::new(1., -2., 3.) / 2., Vector::new(0.5, -1., 1.5));
  }

  #[test]
  fn magnitude() {
    assert_eq!(Vector::new(1., 0., 0.).magnitude(), 1.);
    assert_eq!(Vector::new(0., 1., 0.).magnitude(), 1.);
    assert_eq!(Vector::new(0., 0., 1.).magnitude(), 1.);
    assert_eq!(Vector::new(1., 2., 3.).magnitude(), 14_f64.sqrt());
    assert_eq!(Vector::new(-1., -2., -3.).magnitude(), 14_f64.sqrt());
  }

  #[test]
  fn normalize() {
    assert_eq!(Vector::new(4., 0., 0.).normalize(), Vector::new(1., 0., 0.));
    assert_eq!(
      Vector::new(1., 2., 3.).normalize(),
      Vector::new(1. / 14_f64.sqrt(), 2. / 14_f64.sqrt(), 3. / 14_f64.sqrt(),)
    );
  }

  #[test]
  fn normalize_length() {
    assert_eq!(Vector::new(1., 2., 3.).normalize().magnitude(), 1.);
  }

  #[test]
  fn dot_product() {
    assert_eq!(dot(&Vector::new(1., 2., 3.), &Vector::new(2., 3., 4.)), 20.);
  }

  #[test]
  fn cross_product() {
    let a = Vector::new(1., 2., 3.);
    let b = Vector::new(2., 3., 4.);
    assert_eq!(cross(&a, &b), Vector::new(-1., 2., -1.));
    assert_eq!(cross(&b, &a), Vector::new(1., -2., 1.));
  }
}