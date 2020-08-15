use crate::geometry::{Point, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vector,
}
impl Ray {
  pub fn new(origin: Point, direction: Vector) -> Self {
    return Self { origin, direction };
  }

  pub fn position(&self, t: f64) -> Point {
    return self.origin + self.direction * t;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let origin = Point::new(3., 2., 1.);
    let direction = Vector::new(1., 2., 3.);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
  }

  #[test]
  fn position() {
    let ray = Ray::new(Point::new(2., 3., 4.), Vector::new(1., 0., 0.));
    assert_eq!(ray.position(0.), Point::new(2., 3., 4.,));
    assert_eq!(ray.position(1.), Point::new(3., 3., 4.,));
    assert_eq!(ray.position(-1.), Point::new(1., 3., 4.,));
    assert_eq!(ray.position(2.5), Point::new(4.5, 3., 4.,));
  }
}
