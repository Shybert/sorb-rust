use crate::geometry::Point;
use crate::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PointLight {
  position: Point,
  color: Color,
}
impl PointLight {
  pub fn new(position: Point, color: Color) -> Self {
    return PointLight { position, color };
  }

  pub fn position(&self) -> &Point {
    return &self.position;
  }
  pub fn color(&self) -> &Color {
    return &self.color;
  }
}
impl Default for PointLight {
  fn default() -> Self {
    return Self::new(Point::origin(), Color::white());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn point_light_init_new() {
    let position = Point::new(1., 2., 3.);
    let color = Color::cyan();
    let light = PointLight::new(position, color);
    assert_eq!(light.position(), &position);
    assert_eq!(light.color(), &color);
  }

  #[test]
  fn point_light_init_default() {
    let light = PointLight::default();
    assert_eq!(light.position(), &Point::origin());
    assert_eq!(light.color(), &Color::white());
  }
}
