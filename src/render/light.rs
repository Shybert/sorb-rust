use crate::geometry::Point;
use crate::Color;

pub struct Light {
  position: Point,
  color: Color,
}
impl Light {
  pub fn new(position: Point, color: Color) -> Self {
    return Self { position, color };
  }

  pub fn get_position(&self) -> &Point {
    return &self.position;
  }
  pub fn get_color(&self) -> &Color {
    return &self.color;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init() {
    let position = Point::origin();
    let color = Color::white();
    let light = Light::new(position, color);
    assert_eq!(light.get_position(), &position);
    assert_eq!(light.get_color(), &color);
  }
}
