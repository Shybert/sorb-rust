use crate::Color;

#[derive(Debug)]
pub struct Material {
  color: Color,
  ambience: f64,
  diffuse: f64,
  specular: f64,
  shininess: f64,
}
impl Material {
  pub fn new(color: Color, ambience: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
    return Self {
      color,
      ambience,
      diffuse,
      specular,
      shininess,
    };
  }

  pub fn color(&self) -> &Color {
    return &self.color;
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
    return Self::new(Color::white(), 0.1, 0.9, 0.9, 200.);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let color = Color::red();
    let ambience = 0.3;
    let diffuse = 0.5;
    let specular = 0.7;
    let shininess = 40.;

    let material = Material::new(color, ambience, diffuse, specular, shininess);
    assert_eq!(material.color(), &color);
    assert_eq!(material.ambience(), &ambience);
    assert_eq!(material.diffuse(), &diffuse);
    assert_eq!(material.specular(), &specular);
    assert_eq!(material.shininess(), &shininess);
  }

  #[test]
  fn init_default() {
    let material = Material::default();
    assert_eq!(material.color(), &Color::white());
    assert_eq!(material.ambience(), &0.1);
    assert_eq!(material.diffuse(), &0.9);
    assert_eq!(material.specular(), &0.9);
    assert_eq!(material.shininess(), &200.);
  }
}
