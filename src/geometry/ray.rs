use crate::geometry::{point::Point, vector::Vector};

pub struct Ray {
  origin: Point,
  direction: Vector,
}
impl Ray {
  fn from(origin: Point, direction: Vector) -> Self {
    return Self { origin, direction };
  }

  fn get_origin(&self) -> &Point {
    return &self.origin;
  }
  fn get_direction(&self) -> &Vector {
    return &self.direction;
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
}
