use crate::utils::approx_equals;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    return Self { x, y, z };
  }

  pub fn zero() -> Self {
    return Self::new(0., 0., 0.);
  }

  pub fn magnitude(&self) -> f64 {
    return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
  }

  pub fn normalize(&self) -> Self {
    let magnitude = self.magnitude();
    return Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude);
  }

  pub fn dot(&self, other: &Vector) -> f64 {
    return self.x * other.x + self.y * other.y + self.z * other.z;
  }
  pub fn cross(&self, other: &Vector) -> Vector {
    return Vector::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x,
    );
  }

  pub fn reflect(&self, normal: &Self) -> Self {
    return *self - 2. * self.dot(normal) * *normal;
  }
}

impl Default for Vector {
  fn default() -> Self {
    return Self::zero();
  }
}
impl PartialEq for Vector {
  fn eq(&self, other: &Self) -> bool {
    return approx_equals(self.x, other.x)
      && approx_equals(self.y, other.y)
      && approx_equals(self.z, other.z);
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
    return vector * self;
  }
}
impl Div<f64> for Vector {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    return Self::new(self.x / scalar, self.y / scalar, self.z / scalar);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f64::consts::SQRT_2;

  #[test]
  fn init_new() {
    let vector = Vector::new(1., 2., 3.);
    assert_eq!(vector.x, 1.);
    assert_eq!(vector.y, 2.);
    assert_eq!(vector.z, 3.);
  }

  #[test]
  fn init_zero() {
    assert_eq!(Vector::zero(), Vector::new(0., 0., 0.,));
  }

  #[test]
  fn init_default() {
    assert_eq!(Vector::default(), Vector::zero());
  }

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
    let a = Vector::new(1., 2., 3.);
    let b = Vector::new(2., 3., 4.);
    assert_eq!(a.dot(&b), 20.);
  }

  #[test]
  fn cross_product() {
    let a = Vector::new(1., 2., 3.);
    let b = Vector::new(2., 3., 4.);
    assert_eq!(a.cross(&b), Vector::new(-1., 2., -1.));
    assert_eq!(b.cross(&a), Vector::new(1., -2., 1.));
  }

  #[test]
  fn reflect_at_45_degree_angle() {
    let vector = Vector::new(1., -1., 0.);
    let normal = Vector::new(0., 1., 0.);
    assert_eq!(vector.reflect(&normal), Vector::new(1., 1., 0.));
  }

  #[test]
  fn reflect_off_slanted_surface() {
    let vector = Vector::new(0., -1., 0.);
    let normal = Vector::new(SQRT_2 / 2., SQRT_2 / 2., 0.);
    assert_eq!(vector.reflect(&normal), Vector::new(1., 0., 0.));
  }

  #[test]
  fn reflect_in_3d() {
    let vector = Vector::new(0., -1., 0.);
    let normal = Vector::new(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.);
    assert_eq!(
      vector.reflect(&normal),
      Vector::new(2. / 3., -1. / 3., 2. / 3.)
    );
  }
}
