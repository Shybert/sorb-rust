use crate::geometry::{Point, Ray};
use crate::render::{phong, PointLight};
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

  pub fn is_shadowed(&self, point: &Point, light: &PointLight) -> bool {
    let point_to_light = *light.position() - *point;
    let direction = point_to_light.normalize();
    let distance = point_to_light.magnitude();

    let intersections = &self.intersect(&Ray::new(*point, direction));
    let hit = find_hit(intersections);
    return match hit {
      None => false,
      Some(intersection) => intersection.time <= distance,
    };
  }

  fn shade_hit(&self, hit: &Intersection) -> Color {
    return self
      .lights()
      .iter()
      .map(|light| {
        let light_vector = (*light.position() - hit.point).normalize();
        phong(
          hit.base_color(),
          hit.material.shading_properties(),
          (light_vector, hit.normal, hit.outgoing),
          *light.color(),
          self.is_shadowed(&hit.point_over(), light),
        )
      })
      .sum();
  }
  pub fn color_at(&self, ray: &Ray) -> Color {
    let intersections = self.intersect(&ray);
    let hit = find_hit(&intersections);
    return match hit {
      None => Color::black(),
      Some(intersection) => self.shade_hit(intersection),
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
          Material::new(Box::new(Color::new(0.8, 1., 0.6)), 0.1, 0.7, 0.2, 200.),
          Matrix::identity(),
        )),
        Box::new(Sphere::new(
          Material::new(Box::new(Color::red()), 1., 0., 0., 0.),
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
  fn is_shadowed_nothing_between_point_and_light() {
    let world = test_world();
    assert_eq!(
      world.is_shadowed(&Point::new(0., 10., 0.), &world.lights()[0]),
      false
    );
  }

  #[test]
  fn is_shadowed_object_between_point_and_light() {
    let world = test_world();
    assert_eq!(
      world.is_shadowed(&Point::new(10., -10., 10.), &world.lights()[0]),
      true
    );
  }

  #[test]
  fn is_shadowed_point_between_light_and_object() {
    let world = test_world();
    assert_eq!(
      world.is_shadowed(&Point::new(-5., 5., -5.), &world.lights()[0]),
      false
    );
  }

  #[test]
  fn is_shadowed_light_between_point_and_object() {
    let world = test_world();
    assert_eq!(
      world.is_shadowed(&Point::new(-15., 15., -15.), &world.lights()[0]),
      false
    )
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

  #[test]
  fn color_at_intersection_in_shadow() {
    let world = test_world();
    let ray = Ray::new(Point::new(1., -1., 1.), Vector::new(-1., 1., -1.));
    let color = world.color_at(&ray);
    assert_eq!(color, Color::new(0.08, 0.1, 0.06));
  }
}
