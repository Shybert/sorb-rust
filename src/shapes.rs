use crate::geometry::{Material, Matrix, Point, Ray, Vector};
use crate::utils::EPSILON;
use std::cmp::Ordering::Equal;

mod sphere;
pub use sphere::*;

mod plane;
pub use plane::*;

pub trait Shape {
  fn material(&self) -> &Material;
  fn set_material(&mut self, material: Material);

  fn transformation(&self) -> &Matrix;
  fn set_transformation(&mut self, transformation: Matrix);

  fn intersect_object_space(&self, ray: &Ray) -> Vec<f64>;
  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let ray_object = self.transformation().inverse() * *ray;
    let intersection_times = self.intersect_object_space(&ray_object);
    return intersection_times
      .into_iter()
      .map(|time| {
        let point = ray.position(time);
        return Intersection::new(time, point, self.normal_at(&point), self.material());
      })
      .collect();
  }

  fn normal_at_object_space(&self, point: &Point) -> Vector;
  fn normal_at(&self, point: &Point) -> Vector {
    let point_object = self.transformation().inverse() * *point;
    let normal_object = self.normal_at_object_space(&point_object);
    let normal_world = self.transformation().inverse().transpose() * normal_object;
    return normal_world.normalize();
  }
}

#[derive(Debug)]
pub struct Intersection<'a> {
  pub time: f64,
  pub point: Point,
  pub normal: Vector,
  pub material: &'a Material,
}
impl<'a> Intersection<'a> {
  pub fn new(time: f64, point: Point, normal: Vector, material: &'a Material) -> Self {
    return Self {
      time,
      point,
      normal,
      material,
    };
  }

  pub fn point_over(&self) -> Point {
    return self.point + self.normal * EPSILON;
  }
}

pub fn find_hit<'a>(intersections: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
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
    let normal = Vector::new(1., -1., 1.);

    let intersection = Intersection::new(time, point, normal, &material);
    assert_eq!(intersection.time, time);
    assert_eq!(intersection.point, point);
    assert_eq!(
      intersection.material.color_at(&Point::origin()),
      material.color_at(&Point::origin())
    );
    assert_eq!(intersection.normal, normal);
  }

  #[test]
  fn intersection_point_over() {
    let material = Material::default();
    let point = Point::new(1., 1., 1.);
    let normal = Vector::new(0., 1., 0.);
    let intersection = Intersection::new(0., point, normal, &material);
    assert_eq!(intersection.point_over(), Point::new(1., 1. + EPSILON, 1.));
  }

  fn intersections_time<'a>(times: &[f64], material: &'a Material) -> Vec<Intersection<'a>> {
    return times
      .iter()
      .map(|&time| Intersection::new(time, Point::origin(), Vector::zero(), material))
      .collect();
  }

  #[test]
  fn hit_when_all_positive() {
    let material = Material::default();
    let intersections = intersections_time(&vec![1., 2.], &material);
    let hit = find_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_some_negative() {
    let material = Material::default();
    let intersections = intersections_time(&vec![-1., 1.], &material);
    let hit = find_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_all_negative() {
    let material = Material::default();
    let intersections = intersections_time(&vec![-2., -1.], &material);
    let hit = find_hit(&intersections);
    assert!(hit.is_none());
  }

  #[test]
  fn hit_intersection_order_does_not_matter() {
    let material = Material::default();
    let intersections = intersections_time(&vec![5., 7., -3., 2.], &material);
    let hit = find_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 2.);
  }
}
