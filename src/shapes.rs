use crate::geometry::{Interaction, Matrix, Point, Ray, Vector};
use crate::Color;
use std::cmp::Ordering::Equal;

mod sphere;
pub use sphere::*;

pub struct Material {
  color: Color,
  ambience: f64,
  diffuse: f64,
  specular: f64,
  shininess: f64,
}
impl Material {
  fn new(color: Color, ambience: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
    return Self {
      color,
      ambience,
      diffuse,
      specular,
      shininess,
    };
  }

  pub fn get_color(&self) -> &Color {
    return &self.color;
  }
  pub fn get_ambience(&self) -> &f64 {
    return &self.ambience;
  }
  pub fn get_diffuse(&self) -> &f64 {
    return &self.diffuse;
  }
  pub fn get_specular(&self) -> &f64 {
    return &self.specular;
  }
  pub fn get_shininess(&self) -> &f64 {
    return &self.shininess;
  }
}
impl Default for Material {
  fn default() -> Self {
    return Self::new(Color::white(), 0.1, 0.9, 0.9, 200.);
  }
}

pub trait Shape {
  fn get_transformation(&self) -> &Matrix;
  fn set_transformation(&mut self, transformation: Matrix);
  fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
  fn normal_at(&self, point: &Point) -> Vector;
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

  #[test]
  fn material_init_new() {
    let color = Color::red();
    let ambience = 0.3;
    let diffuse = 0.5;
    let specular = 0.7;
    let shininess = 40.;

    let material = Material::new(color, ambience, diffuse, specular, shininess);
    assert_eq!(material.get_color(), &color);
    assert_eq!(material.get_ambience(), &ambience);
    assert_eq!(material.get_diffuse(), &diffuse);
    assert_eq!(material.get_specular(), &specular);
    assert_eq!(material.get_shininess(), &shininess);
  }

  #[test]
  fn material_init_default() {
    let material = Material::default();
    assert_eq!(material.get_color(), &Color::white());
    assert_eq!(material.get_ambience(), &0.1);
    assert_eq!(material.get_diffuse(), &0.9);
    assert_eq!(material.get_specular(), &0.9);
    assert_eq!(material.get_shininess(), &200.);
  }

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
