use crate::geometry::Point;
use crate::textures::Texture;
use crate::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Stripes {
  a: Color,
  b: Color,
}
impl Stripes {
  pub fn new(a: Color, b: Color) -> Self {
    return Self { a, b };
  }

  pub fn colors(&self) -> (&Color, &Color) {
    return (&self.a, &self.b);
  }
}
impl Texture for Stripes {
  fn color_at(&self, point: &Point) -> Color {
    return if point.x.rem_euclid(2.).floor() == 0. {
      self.a
    } else {
      self.b
    };
  }
}
impl Default for Stripes {
  fn default() -> Self {
    return Self::new(Color::white(), Color::black());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let stripes = Stripes::new(Color::cyan(), Color::red());
    assert_eq!(stripes.colors(), (&Color::cyan(), &Color::red()));
  }

  #[test]
  fn init_default() {
    let stripes = Stripes::default();
    assert_eq!(stripes.colors(), (&Color::white(), &Color::black()));
  }

  #[test]
  fn equality_identical() {
    let color_a = Color::new(0.3, 0.3, 0.6);
    let color_b = Color::new(0.6, 0.2, 0.1);
    let a = Stripes::new(color_a, color_b);
    let b = Stripes::new(color_a, color_b);

    assert_eq!(a, b);

    let b = Stripes::new(color_a, Color::new(0.6000001, 0.2, 0.1));
    assert_eq!(a, b);
  }

  #[test]
  fn equality_different() {
    let a = Stripes::new(Color::yellow(), Color::cyan());
    let b = Stripes::new(Color::red(), Color::green());
    assert_ne!(a, b);
  }

  #[test]
  fn color_at_alternates_in_x() {
    let stripes = Stripes::default();
    assert_eq!(stripes.color_at(&Point::origin()), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0.9, 0., 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(1., 0., 0.)), Color::black());
    assert_eq!(stripes.color_at(&Point::new(-0.1, 0., 0.)), Color::black());
    assert_eq!(stripes.color_at(&Point::new(-1., 0., 0.)), Color::black());
    assert_eq!(stripes.color_at(&Point::new(-1.1, 0., 0.)), Color::white());
  }

  #[test]
  fn color_at_constant_in_y() {
    let stripes = Stripes::default();
    assert_eq!(stripes.color_at(&Point::origin()), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0.9, 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 1., 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., -0.1, 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., -1., 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., -1.1, 0.)), Color::white());
  }

  #[test]
  fn color_at_constant_in_z() {
    let stripes = Stripes::default();
    assert_eq!(stripes.color_at(&Point::origin()), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., 0.9)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., 1.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., -0.1)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., -1.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., -1.1)), Color::white());
  }
}
