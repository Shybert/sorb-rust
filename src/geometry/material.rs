use crate::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
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
}
