use crate::geometry::Ray;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersection_init() {
    let intersection = Intersection::new(5.);
    assert_eq!(intersection.time, 5.)
  }
}
