use crate::geometry::Point;
use crate::Color;
use std::fmt::Debug;

mod pattern;
pub use pattern::*;

pub trait Texture: Debug {
  fn color_at(&self, point: &Point) -> Color;
}

impl Texture for Color {
  fn color_at(&self, _point: &Point) -> Color {
    return *self;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color_color_at_is_constant() {
    let color = Color::default();
    assert_eq!(color.color_at(&Point::origin()), color);
    assert_eq!(color.color_at(&Point::new(-15., 30., 8.)), color);
    assert_eq!(color.color_at(&Point::new(1.36, -32., 12.)), color);
    assert_eq!(color.color_at(&Point::new(0., 33.33, -5.7)), color);
  }
}
