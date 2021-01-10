use crate::geometry::Point;
use crate::patterns::Pattern;
use crate::Color;

#[derive(Debug)]
pub struct Material {
  pattern: Box<dyn Pattern>,
  ambience: f64,
  diffuse: f64,
  specular: f64,
  shininess: f64,
}
impl Material {
  pub fn new(
    pattern: Box<dyn Pattern>,
    ambience: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
  ) -> Self {
    return Self {
      pattern,
      ambience,
      diffuse,
      specular,
      shininess,
    };
  }

  pub fn color_at(&self, point: &Point) -> Color {
    return self.pattern.color_at(point);
  }

  pub fn ambience(&self) -> &f64 {
    return &self.ambience;
  }
  pub fn diffuse(&self) -> &f64 {
    return &self.diffuse;
  }
  pub fn specular(&self) -> &f64 {
    return &self.specular;
  }
  pub fn shininess(&self) -> &f64 {
    return &self.shininess;
  }
}
impl Default for Material {
  fn default() -> Self {
    return Self::new(Box::new(Color::white()), 0.1, 0.9, 0.9, 200.);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let material = Material::new(Box::new(Color::cyan()), 0.1, 0.3, 0.8, 100.);
    assert_eq!(material.color_at(&Point::origin()), Color::cyan());
    assert_eq!(material.ambience(), &0.1);
    assert_eq!(material.diffuse(), &0.3);
    assert_eq!(material.specular(), &0.8);
    assert_eq!(material.shininess(), &100.);
  }

  #[test]
  fn init_default() {
    let material = Material::default();
    assert_eq!(material.color_at(&Point::origin()), Color::white());
    assert_eq!(material.ambience(), &0.1);
    assert_eq!(material.diffuse(), &0.9);
    assert_eq!(material.specular(), &0.9);
    assert_eq!(material.shininess(), &200.);
  }
}
