use crate::geometry::Point;
use crate::textures::Texture;
use crate::utils::Lerp;
use crate::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Gradient {
  a: Color,
  b: Color,
}
impl Gradient {
  pub fn new(a: Color, b: Color) -> Self {
    return Self { a, b };
  }

  pub fn colors(&self) -> (&Color, &Color) {
    return (&self.a, &self.b);
  }
}
impl Texture for Gradient {
  fn color_at(&self, point: &Point) -> Color {
    if point.x.abs().rem_euclid(2.).floor() == 0. {
      return self.a.lerp(&self.b, &point.x.abs().fract());
    } else {
      return self.b.lerp(&self.a, &point.x.abs().fract());
    }
  }
}
impl Default for Gradient {
  fn default() -> Self {
    return Self::new(Color::white(), Color::black());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let gradient = Gradient::new(Color::cyan(), Color::red());
    assert_eq!(gradient.colors(), (&Color::cyan(), &Color::red()));
  }

  #[test]
  fn init_default() {
    let gradient = Gradient::default();
    assert_eq!(gradient.colors(), (&Color::white(), &Color::black()));
  }

  #[test]
  fn equality_identical() {
    let color_a = Color::new(0.3, 0.3, 0.6);
    let color_b = Color::new(0.6, 0.2, 0.1);
    let a = Gradient::new(color_a, color_b);
    let b = Gradient::new(color_a, color_b);

    assert_eq!(a, b);

    let b = Gradient::new(color_a, Color::new(0.6000001, 0.2, 0.1));
    assert_eq!(a, b);
  }

  #[test]
  fn equality_different() {
    let a = Gradient::new(Color::yellow(), Color::cyan());
    let b = Gradient::new(Color::red(), Color::green());
    assert_ne!(a, b);
  }

  #[test]
  fn color_at_linearly_interpolates_in_x() {
    let gradient = Gradient::default();
    assert_eq!(gradient.color_at(&Point::origin()), Color::white());
    assert_eq!(
      gradient.color_at(&Point::new(0.25, 0., 0.)),
      Color::new(0.75, 0.75, 0.75)
    );
    assert_eq!(
      gradient.color_at(&Point::new(0.5, 0., 0.)),
      Color::new(0.5, 0.5, 0.5)
    );
    assert_eq!(
      gradient.color_at(&Point::new(0.75, 0., 0.)),
      Color::new(0.25, 0.25, 0.25)
    );
  }

  #[test]
  fn color_at_is_continuous() {
    let gradient = Gradient::default();
    assert_eq!(
      gradient.color_at(&Point::new(1.25, 1.25, 1.25)),
      Color::new(0.25, 0.25, 0.25)
    );
    assert_eq!(
      gradient.color_at(&Point::new(0.75, 0., 0.)),
      Color::new(0.25, 0.25, 0.25)
    );

    assert_eq!(
      gradient.color_at(&Point::new(0.25, 0., 0.)),
      Color::new(0.75, 0.75, 0.75)
    );
    assert_eq!(
      gradient.color_at(&Point::new(-0.25, 0., 0.)),
      Color::new(0.75, 0.75, 0.75)
    );
  }

  #[test]
  fn color_at_constant_in_y() {
    let gradient = Gradient::default();
    assert_eq!(gradient.color_at(&Point::origin()), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., 0.9, 0.)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., 1., 0.)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., -0.1, 0.)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., -1., 0.)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., -1.1, 0.)), Color::white());
  }

  #[test]
  fn color_at_constant_in_z() {
    let gradient = Gradient::default();
    assert_eq!(gradient.color_at(&Point::origin()), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., 0., 0.9)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., 0., 1.)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., 0., -0.1)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., 0., -1.)), Color::white());
    assert_eq!(gradient.color_at(&Point::new(0., 0., -1.1)), Color::white());
  }
}
