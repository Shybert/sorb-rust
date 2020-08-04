use crate::geometry::Ray;
use std::cmp::Ordering::Equal;

mod sphere;
pub use sphere::*;

pub trait Shape {
  fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
  pub time: f64,
}
impl Intersection {
  pub fn new(time: f64) -> Self {
    return Self { time };
  }
}
pub fn get_hit(intersections: &[Intersection]) -> Option<&Intersection> {
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
    let intersection = Intersection::new(5.);
    assert_eq!(intersection.time, 5.)
  }

  #[test]
  fn hit_when_all_positive() {
    let intersections = vec![Intersection::new(1.), Intersection::new(2.)];
    let hit = get_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_some_negative() {
    let intersections = vec![Intersection::new(-1.), Intersection::new(1.)];
    let hit = get_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_all_negative() {
    let intersections = vec![Intersection::new(-2.), Intersection::new(-1.)];
    let hit = get_hit(&intersections);
    assert!(hit.is_none());
  }

  #[test]
  fn hit_intersection_order_does_not_matter() {
    let intersections = vec![
      Intersection::new(5.),
      Intersection::new(7.),
      Intersection::new(-3.),
      Intersection::new(2.),
    ];
    let hit = get_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 2.);
  }
}
