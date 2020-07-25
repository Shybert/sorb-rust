use crate::geometry::{Point, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
  origin: Point,
  direction: Vector,
}
impl Ray {
  pub fn from(origin: Point, direction: Vector) -> Self {
    return Self { origin, direction };
  }

  pub fn get_origin(&self) -> &Point {
    return &self.origin;
  }
  pub fn get_direction(&self) -> &Vector {
    return &self.direction;
  }

  pub fn position(&self, t: f64) -> Point {
    return *self.get_origin() + *self.get_direction() * t;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_origin() {
    let origin = Point::from(1., 2., 3.);
    let direction = Vector::from(4., 5., 6.);
    let ray = Ray::from(origin, direction);
    assert_eq!(ray.get_origin(), &origin);
  }
  #[test]
  fn get_direction() {
    let origin = Point::from(1., 2., 3.);
    let direction = Vector::from(4., 5., 6.);
    let ray = Ray::from(origin, direction);
    assert_eq!(ray.get_direction(), &direction);
  }

  #[test]
  fn position() {
    let ray = Ray::from(Point::from(2., 3., 4.), Vector::from(1., 0., 0.));
    assert_eq!(ray.position(0.), Point::from(2., 3., 4.,));
    assert_eq!(ray.position(1.), Point::from(3., 3., 4.,));
    assert_eq!(ray.position(-1.), Point::from(1., 3., 4.,));
    assert_eq!(ray.position(2.5), Point::from(4.5, 3., 4.,));
  }
}
