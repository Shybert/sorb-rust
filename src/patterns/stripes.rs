use crate::geometry::Point;
use crate::patterns::Pattern;
use crate::Color;

pub struct Stripes {
  a: Color,
  b: Color,
}
impl Stripes {
  fn new(a: Color, b: Color) -> Self {
    return Self { a, b };
  }
}
impl Pattern for Stripes {
  fn get_colors(&self) -> Vec<&Color> {
    return vec![&self.a, &self.b];
  }

  fn color_at(&self, point: &Point) -> Color {
    return if point.x.rem_euclid(2.).floor() == 0. {
      self.a
    } else {
      self.b
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let stripes = Stripes::new(Color::cyan(), Color::red());
    let colors = stripes.get_colors();
    assert_eq!(colors.len(), 2);
    assert_eq!(colors[0], &Color::cyan());
    assert_eq!(colors[1], &Color::red());
  }

  #[test]
  fn color_at_alternates_in_x() {
    let stripes = Stripes::new(Color::white(), Color::black());
    assert_eq!(stripes.color_at(&Point::origin()), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0.9, 0., 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(1., 0., 0.)), Color::black());
    assert_eq!(stripes.color_at(&Point::new(-0.1, 0., 0.)), Color::black());
    assert_eq!(stripes.color_at(&Point::new(-1., 0., 0.)), Color::black());
    assert_eq!(stripes.color_at(&Point::new(-1.1, 0., 0.)), Color::white());
  }

  #[test]
  fn color_at_constant_in_y() {
    let stripes = Stripes::new(Color::white(), Color::black());
    assert_eq!(stripes.color_at(&Point::origin()), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0.9, 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 1., 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., -0.1, 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., -1., 0.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., -1.1, 0.)), Color::white());
  }

  #[test]
  fn color_at_constant_in_z() {
    let stripes = Stripes::new(Color::white(), Color::black());
    assert_eq!(stripes.color_at(&Point::origin()), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., 0.9)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., 1.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., -0.1)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., -1.)), Color::white());
    assert_eq!(stripes.color_at(&Point::new(0., 0., -1.1)), Color::white());
  }
}
