use crate::geometry::Ray;
use crate::render::{lighting, PointLight};
use crate::shapes::{find_hit, Intersection, Shape};
use crate::Color;
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

  pub fn color_at(&self, ray: &Ray) -> Color {
    let intersections = self.intersect(&ray);
    let hit = find_hit(&intersections);
    return match hit {
      None => Color::black(),
      Some(intersection) => {
        let position = ray.position(intersection.time);
        let eye_vector = (ray.origin - position).normalize();
        lighting(
          intersection.material,
          position,
          self.lights()[0],
          eye_vector,
          intersection.normal,
        )
      }
    };
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
          Material::new(Color::new(0.8, 1., 0.6), 0.1, 0.7, 0.2, 200.),
          Matrix::identity(),
        )),
        Box::new(Sphere::new(
          Material::new(Color::red(), 1., 0., 0., 0.),
          Matrix::identity().scale(0.5, 0.5, 0.5),
        )),
        Box::new(Sphere::new(
          Material::default(),
          Matrix::identity().translate(0., -5., 0.),
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
    let ray = Ray::new(Point::new(0., -2., 0.), Vector::new(0., 1., 0.));
    let intersections = world.intersect(&ray);
    assert_eq!(intersections.len(), 6);
    assert_eq!(intersections[0].time, -4.);
    assert_eq!(intersections[1].time, -2.);
    assert_eq!(intersections[2].time, 1.);
    assert_eq!(intersections[3].time, 1.5);
    assert_eq!(intersections[4].time, 2.5);
    assert_eq!(intersections[5].time, 3.);
  }

  #[test]
  fn color_at_ray_misses() {
    let world = test_world();
    let ray = Ray::new(Point::new(0., 0., -2.), Vector::new(0., 1., 0.));
    let color = world.color_at(&ray);
    assert_eq!(color, Color::black());
  }

  #[test]
  fn color_at_ray_hits() {
    let world = test_world();
    let ray = Ray::new(Point::new(0., 0., -2.), Vector::new(0., 0., 1.));
    let color = world.color_at(&ray);
    assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
  }

  #[test]
  fn color_at_intersection_behind_and_in_front() {
    let world = test_world();
    let ray = Ray::new(Point::new(0., 0., 0.75), Vector::new(0., 0., -1.));
    let color = world.color_at(&ray);
    assert_eq!(color, Color::red());
  }
}
