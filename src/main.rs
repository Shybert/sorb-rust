fn main() {
    println!("Hello, world!");
}

use std::ops::{Add, Mul, Neg, Sub};

fn approx_equals(a: f64, b: f64) -> bool {
    return (a - b).abs() < 0.00001;
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}
impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        return Point { x, y, z };
    }
}

impl PartialEq<Self> for Point {
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
        return Point::new(self * point.x, self * point.y, self * point.z);
    }
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
impl Vector {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Vector {
        return Vector { x, y, z, w };
    }
}

impl PartialEq<Self> for Vector {
    fn eq(&self, other: &Self) -> bool {
        return approx_equals(self.x, other.x)
            && approx_equals(self.y, other.y)
            && approx_equals(self.z, other.z)
            && approx_equals(self.w, other.w);
    }
}
impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        );
    }
}
impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        );
    }
}
impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        return Self::new(-self.x, -self.y, -self.z, -self.w);
    }
}
impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        return Self::new(
            scalar * self.x,
            scalar * self.y,
            scalar * self.z,
            scalar * self.w,
        );
    }
}
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        return Vector::new(
            self * vector.x,
            self * vector.y,
            self * vector.z,
            self * vector.w,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_equality() {
        assert_eq!(Point::new(4.0, -4.0, 3.0), Point::new(4.0, -4.0, 3.0));

        assert_eq!(Point::new(1.000000001, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn point_addition() {
        assert_eq!(
            Point::new(3.0, -2.0, 5.0) + Point::new(-2.0, 3.0, 1.0),
            Point::new(1.0, 1.0, 6.0)
        )
    }

    #[test]
    fn point_subtraction() {
        assert_eq!(
            Point::new(3.0, 2.0, 1.0) - Point::new(5.0, 6.0, 7.0),
            Point::new(-2.0, -4.0, -6.0)
        )
    }

    #[test]
    fn point_negation() {
        assert_eq!(-Point::new(1.0, -2.0, 3.0), Point::new(-1.0, 2.0, -3.0))
    }

    #[test]
    fn point_scalar_multiplication() {
        assert_eq!(
            Point::new(1.0, -2.0, 3.0) * 3.5,
            Point::new(3.5, -7.0, 10.5)
        );
        assert_eq!(
            3.5 * Point::new(1.0, -2.0, 3.0),
            Point::new(3.5, -7.0, 10.5)
        );

        assert_eq!(0.5 * Point::new(1.0, -2.0, 3.0), Point::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn vector_equality() {
        assert_eq!(
            Vector::new(4.0, -4.0, 3.0, -42.0),
            Vector::new(4.0, -4.0, 3.0, -42.0)
        );

        assert_eq!(
            Vector::new(1.000000001, 0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn vector_addition() {
        assert_eq!(
            Vector::new(3.0, -2.0, 5.0, 0.0) + Vector::new(-2.0, 3.0, 1.0, 0.0),
            Vector::new(1.0, 1.0, 6.0, 0.0)
        )
    }

    #[test]
    fn vector_subtraction() {
        assert_eq!(
            Vector::new(3.0, 2.0, 1.0, 0.0) - Vector::new(5.0, 6.0, 7.0, 0.0),
            Vector::new(-2.0, -4.0, -6.0, 0.0)
        )
    }

    #[test]
    fn vector_negation() {
        assert_eq!(
            -Vector::new(1.0, -2.0, 3.0, -4.0),
            Vector::new(-1.0, 2.0, -3.0, 4.0)
        )
    }

    #[test]
    fn vector_scalar_multiplication() {
        assert_eq!(
            Vector::new(1.0, -2.0, 3.0, -4.0) * 3.5,
            Vector::new(3.5, -7.0, 10.5, -14.0)
        );
        assert_eq!(
            3.5 * Vector::new(1.0, -2.0, 3.0, -4.0),
            Vector::new(3.5, -7.0, 10.5, -14.0)
        );

        assert_eq!(
            0.5 * Vector::new(1.0, -2.0, 3.0, -4.0),
            Vector::new(0.5, -1.0, 1.5, -2.0)
        );
    }
}
