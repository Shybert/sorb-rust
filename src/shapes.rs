use crate::geometry::{Material, Matrix, Point, Ray, Vector};
use crate::utils::EPSILON;
use crate::Color;
use std::cmp::Ordering::Equal;

mod sphere;
pub use sphere::*;

mod plane;
pub use plane::*;

pub trait Shape {
  fn material(&self) -> &Material;
  fn set_material(&mut self, material: Material);

  fn object_to_world(&self) -> &Matrix;
  fn set_object_to_world(&mut self, object_to_world: Matrix);

  fn intersect_object_space(&self, ray: &Ray) -> Vec<f64>;
  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let ray_object = self.object_to_world().inverse() * *ray;
    let intersection_times = self.intersect_object_space(&ray_object);
    return intersection_times
      .into_iter()
      .map(|time| {
        let point = ray.position(time);
        return Intersection::new(
          time,
          point,
          -ray.direction.normalize(),
          self.normal_at(&point),
          self.material(),
        );
      })
      .collect();
  }

  fn normal_at_object_space(&self, point: &Point) -> Vector;
  fn normal_at(&self, point: &Point) -> Vector {
    let point_object = self.object_to_world().inverse() * *point;
    let normal_object = self.normal_at_object_space(&point_object);
    let normal_world = self.object_to_world().inverse().transpose() * normal_object;
    return normal_world.normalize();
  }
}

#[derive(Debug)]
pub struct Intersection<'a> {
  pub time: f64,
  pub point: Point,
  pub outgoing: Vector,
  pub normal: Vector,
  pub material: &'a Material,
}
impl<'a> Intersection<'a> {
  pub fn new(
    time: f64,
    point: Point,
    outgoing: Vector,
    normal: Vector,
    material: &'a Material,
  ) -> Self {
    return Self {
      time,
      point,
      outgoing,
      normal,
      material,
    };
  }

  /// Returns the base color at the intersection point, before shading is applied.
  pub fn base_color(&self) -> Color {
    return self.material.color_at(&self.point);
  }

  /// Returns the intersection point shifted [`EPSILON`](EPSILON) in the direction of the normal vector.
  ///
  /// Used to prevent shadow acne.
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
  use crate::textures::Stripes;

  #[test]
  fn intersection_init() {
    let time = 5.;
    let point = Point::new(-1., 1., -1.);
    let outgoing = Vector::new(0., 1., 0.);
    let material = Material::default();
    let normal = Vector::new(1., -1., 1.);

    let intersection = Intersection::new(time, point, outgoing, normal, &material);
    assert_eq!(intersection.time, time);
    assert_eq!(intersection.point, point);
    assert_eq!(intersection.outgoing, outgoing);
    assert_eq!(
      intersection.material.color_at(&Point::origin()),
      material.color_at(&Point::origin())
    );
    assert_eq!(intersection.normal, normal);
  }

  fn test_base_color(material: Material, point: Point) {
    let intersection = Intersection::new(0., point, Vector::zero(), Vector::zero(), &material);
    assert_eq!(intersection.base_color(), material.color_at(&point));
  }

  #[test]
  fn intersection_base_color() {
    test_base_color(Material::default(), Point::origin());
  }

  #[test]
  fn intersection_base_color_stripes_texture() {
    test_base_color(
      Material::new(Box::new(Stripes::default()), 0., 0., 0., 0.),
      Point::new(1.5, 0., 0.),
    );
  }

  #[test]
  fn intersection_point_over() {
    let material = Material::default();
    let point = Point::new(1., 1., 1.);
    let normal = Vector::new(0., 1., 0.);

    let intersection = Intersection::new(0., point, Vector::zero(), normal, &material);
    assert_eq!(intersection.point_over(), Point::new(1., 1. + EPSILON, 1.));
  }

  fn test_find_hit(times: &[f64], expected_hit_time: Option<f64>) {
    let material = Material::default();
    let intersections: Vec<Intersection> = times
      .iter()
      .map(|&time| {
        Intersection::new(
          time,
          Point::origin(),
          Vector::zero(),
          Vector::zero(),
          &material,
        )
      })
      .collect();

    let hit = find_hit(&intersections);
    match expected_hit_time {
      Some(expected_time) => assert_eq!(hit.expect("Expected hit").time, expected_time),
      None => assert!(hit.is_none()),
    }
  }

  #[test]
  fn hit_when_all_positive() {
    let times = vec![1., 2.];
    let expected_hit_time = Some(1.);
    test_find_hit(&times, expected_hit_time);
  }

  #[test]
  fn hit_when_some_negative() {
    let times = vec![-1., 1.];
    let expected_hit_time = Some(1.);
    test_find_hit(&times, expected_hit_time);
  }

  #[test]
  fn hit_when_all_negative() {
    let times = vec![-2., -1.];
    let expected_hit_time = None;
    test_find_hit(&times, expected_hit_time);
  }

  #[test]
  fn hit_intersection_order_does_not_matter() {
    let times = vec![5., 7., -3., 2.];
    let expected_hit_time = Some(2.);
    test_find_hit(&times, expected_hit_time);
  }
}
