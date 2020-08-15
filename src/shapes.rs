use crate::geometry::{Material, Matrix, Point, Ray, Vector};
use std::cmp::Ordering::Equal;

mod sphere;
pub use sphere::*;

pub trait Shape {
  fn material(&self) -> &Material;
  fn set_material(&mut self, material: Material);

  fn transformation(&self) -> &Matrix;
  fn set_transformation(&mut self, transformation: Matrix);

  fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
  fn normal_at(&self, point: &Point) -> Vector;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
  pub time: f64,
  pub material: Material,
  pub normal: Vector,
}
impl Intersection {
  pub fn new(time: f64, material: Material, normal: Vector) -> Self {
    return Self {
      time,
      material,
      normal,
    };
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
    let material = Material::default();
    let vector = Vector::new(1., -1., 1.);

    let intersection = Intersection::new(time, material, vector);
    assert_eq!(intersection.time, time);
    assert_eq!(intersection.material, material);
    assert_eq!(intersection.normal, vector);
  }

  fn intersection_time(time: f64) -> Intersection {
    return Intersection::new(time, Material::default(), Vector::zero());
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
