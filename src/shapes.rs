use crate::geometry::{Interaction, Matrix, Ray};
use std::cmp::Ordering::Equal;

mod sphere;
pub use sphere::*;

pub trait Shape {
  fn get_transformation(&self) -> &Matrix;
  fn set_transformation(&mut self, transformation: Matrix);
  fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
  pub time: f64,
  pub interaction: Interaction,
}
impl Intersection {
  pub fn new(time: f64, interaction: Interaction) -> Self {
    return Self { time, interaction };
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
  use crate::color::Color;

  #[test]
  fn intersection_init() {
    let interaction = Interaction::new(Color::new(1., 0., 1.));
    let intersection = Intersection::new(5., interaction);
    assert_eq!(intersection.time, 5.);
    assert_eq!(intersection.interaction, interaction);
  }

  fn intersection_time(time: f64) -> Intersection {
    return Intersection::new(time, Interaction::new(Color::default()));
  }

  #[test]
  fn hit_when_all_positive() {
    let intersections = vec![intersection_time(1.), intersection_time(2.)];
    let hit = get_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_some_negative() {
    let intersections = vec![intersection_time(-1.), intersection_time(1.)];
    let hit = get_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 1.);
  }

  #[test]
  fn hit_when_all_negative() {
    let intersections = vec![intersection_time(-2.), intersection_time(-1.)];
    let hit = get_hit(&intersections);
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
    let hit = get_hit(&intersections).expect("Expected hit");
    assert_eq!(hit.time, 2.);
  }
}
