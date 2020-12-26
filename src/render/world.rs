use crate::geometry::Ray;
use crate::render::PointLight;
use crate::shapes::{Intersection, Shape};
use std::cmp::Ordering::Equal;

#[derive(Default)]
pub struct World {
  objects: Vec<Box<dyn Shape>>,
  lights: Vec<PointLight>,
}
impl World {
  pub fn new(objects: Vec<Box<dyn Shape>>, lights: Vec<PointLight>) -> Self {
    return Self { objects, lights };
  }

  pub fn objects(&self) -> &[Box<dyn Shape>] {
    return &self.objects;
  }

  pub fn lights(&self) -> &[PointLight] {
    return &self.lights;
  }

  pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let mut intersections: Vec<Intersection> = self
      .objects()
      .iter()
      .flat_map(|object| object.intersect(&ray))
      .collect();
    intersections.sort_unstable_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Equal));
    return intersections;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::geometry::{Material, Matrix, Point, Vector};
  use crate::render::PointLight;
  use crate::shapes::Sphere;
  use crate::Color;

  fn test_world() -> World {
    return World::new(
      vec![
        Box::new(Sphere::new(
          Material::new(Color::new(0.8, 1.2, 0.6), 0.1, 0.7, 0.2, 20.),
          Matrix::identity(),
        )),
        Box::new(Sphere::new(
          Material::default(),
          Matrix::identity().scale(0.5, 0.5, 0.5),
        )),
      ],
      vec![PointLight::new(
        Point::new(-10., 10., -10.),
        Color::new(1., 1., 1.),
      )],
    );
  }

  #[test]
  fn init_new() {
    let world = World::new(
      vec![Box::new(Sphere::default()), Box::new(Sphere::default())],
      vec![PointLight::default(); 3],
    );
    assert_eq!(world.objects().len(), 2);
    assert_eq!(world.lights().len(), 3);
  }

  #[test]
  fn init_default() {
    let world = World::default();
    assert_eq!(world.objects().len(), 0);
    assert_eq!(world.lights().len(), 0);
  }

  #[test]
  fn intersect() {
    let world = test_world();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = world.intersect(&ray);
    assert_eq!(intersections.len(), 4);
    assert_eq!(intersections[0].time, 4.);
    assert_eq!(intersections[1].time, 4.5);
    assert_eq!(intersections[2].time, 5.5);
    assert_eq!(intersections[3].time, 6.);
  }
}
