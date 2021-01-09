use crate::geometry::Point;
use crate::patterns::Pattern;
use crate::Color;

#[derive(Debug)]
enum MaterialColor {
  Color(Color),
  Pattern(Box<dyn Pattern>),
}

#[derive(Debug)]
pub struct Material {
  material_color: MaterialColor,
  ambience: f64,
  diffuse: f64,
  specular: f64,
  shininess: f64,
}
impl Material {
  pub fn with_color(
    color: Color,
    ambience: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
  ) -> Self {
    return Self {
      material_color: MaterialColor::Color(color),
      ambience,
      diffuse,
      specular,
      shininess,
    };
  }
  pub fn with_pattern(
    pattern: Box<dyn Pattern>,
    ambience: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
  ) -> Self {
    return Self {
      material_color: MaterialColor::Pattern(pattern),
      ambience,
      diffuse,
      specular,
      shininess,
    };
  }

  pub fn color_at(&self, point: &Point) -> Color {
    return match &self.material_color {
      MaterialColor::Color(color) => *color,
      MaterialColor::Pattern(pattern) => pattern.color_at(point),
    };
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
    return Self::with_color(Color::white(), 0.1, 0.9, 0.9, 200.);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::patterns::Stripes;

  #[test]
  fn init_with_color() {
    let material = Material::with_color(Color::cyan(), 0.1, 0.3, 0.8, 100.);
    assert_eq!(material.color_at(&Point::origin()), Color::cyan());
    assert_eq!(material.ambience(), &0.1);
    assert_eq!(material.diffuse(), &0.3);
    assert_eq!(material.specular(), &0.8);
    assert_eq!(material.shininess(), &100.);
  }

  #[test]
  fn init_with_pattern() {
    let material = Material::with_pattern(Box::new(Stripes::default()), 0.3, 0.8, 0.7, 150.);
    assert_eq!(material.color_at(&Point::new(1., 0., 0.)), Color::black());
    assert_eq!(material.ambience(), &0.3);
    assert_eq!(material.diffuse(), &0.8);
    assert_eq!(material.specular(), &0.7);
    assert_eq!(material.shininess(), &150.);
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
