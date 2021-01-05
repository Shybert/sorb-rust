use crate::geometry::{Material, Matrix, Point, Ray, Vector};
use crate::utils::EPSILON;
use std::cmp::Ordering::Equal;

mod sphere;
pub use sphere::*;

pub trait Shape {
  fn material(&self) -> &Material;
  fn set_material(&mut self, material: Material);

  fn transformation(&self) -> &Matrix;
  fn set_transformation(&mut self, transformation: Matrix);

  fn intersect(&self, ray: &Ray) -> Vec<Intersection>;

  fn normal_at_object_space(&self, point: &Point) -> Vector;
  fn normal_at(&self, point: &Point) -> Vector {
    let point_object = self.transformation().inverse() * *point;
    let normal_object = self.normal_at_object_space(&point_object);
    let normal_world = self.transformation().inverse().transpose() * normal_object;
    return normal_world.normalize();
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
  pub time: f64,
  pub point: Point,
  pub material: Material,
  pub normal: Vector,
}
impl Intersection {
  pub fn new(time: f64, point: Point, material: Material, normal: Vector) -> Self {
    return Self {
      time,
      point,
      material,
      normal,
    };
  }

  pub fn point_over(&self) -> Point {
    return self.point + self.normal * EPSILON;
  }
}

pub fn find_hit(intersections: &[Intersection]) -> Option<&Intersection> {
  return intersections
    .iter()
    .filter(|intersection| intersection.time >= 0.)
    .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap_or(Equal));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersection_init() {
    let time = 5.;
    let point = Point::new(-1., 1., -1.);
    let material = Material::default();
    let vector = Vector::new(1., -1., 1.);

    let intersection = Intersection::new(time, point, material, vector);
    assert_eq!(intersection.time, time);
    assert_eq!(intersection.point, point);
    assert_eq!(intersection.material, material);
    assert_eq!(intersection.normal, vector);
  }

  #[test]
  fn intersection_point_over() {
    let point = Point::new(1., 1., 1.);
    let normal = Vector::new(0., 1., 0.);
    let intersection = Intersection::new(0., point, Material::default(), normal);
    assert_eq!(intersection.point_over(), Point::new(1., 1. + EPSILON, 1.));
  }

  fn intersection_time(time: f64) -> Intersection {
    return Intersection::new(time, Point::origin(), Material::default(), Vector::zero());
  }

  #[test]
  fn hit_when_all_positive() {
    let intersections = vec![intersection_time(1.), intersection_time(2.)];
    let hit = find_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_some_negative() {
    let intersections = vec![intersection_time(-1.), intersection_time(1.)];
    let hit = find_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_all_negative() {
    let intersections = vec![intersection_time(-2.), intersection_time(-1.)];
    let hit = find_hit(&intersections);
    assert!(hit.is_none());
  }

  #[test]
  fn hit_intersection_order_does_not_matter() {
    let intersections = vec![
      intersection_time(5.),
      intersection_time(7.),
      intersection_time(-3.),
      intersection_time(2.),
    ];
    let hit = find_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 2.);
  }
}
