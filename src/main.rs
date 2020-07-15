fn main() {
    println!("Hello, world!");
}

use std::ops::Add;

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

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
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

impl PartialEq<Vector> for Vector {
    fn eq(&self, other: &Vector) -> bool {
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
}
