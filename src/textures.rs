use crate::geometry::{Matrix, Point};
use crate::Color;
use std::fmt::Debug;

mod pattern;
pub use pattern::*;

pub trait Texture: Debug {
  fn texture_to_world(&self) -> &Matrix;

  fn color_at_texture_space(&self, point: &Point) -> Color;
  fn color_at(&self, point: &Point) -> Color {
    let point_texture = self.texture_to_world().inverse() * *point;
    return self.color_at_texture_space(&point_texture);
  }
}

impl Texture for Color {
  fn texture_to_world(&self) -> &Matrix {
    panic!("Colors can not have a transformation.");
  }

  fn color_at_texture_space(&self, _point: &Point) -> Color {
    return *self;
  }
  fn color_at(&self, point: &Point) -> Color {
    return self.color_at_texture_space(point);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "transformation")]
  fn color_texture_to_world_always_panics() {
    let color = Color::default();
    color.texture_to_world();
  }

  #[test]
  fn color_color_at_is_constant() {
    let color = Color::default();
    assert_eq!(color.color_at(&Point::origin()), Color::black());
    assert_eq!(color.color_at(&Point::new(-15., 30., 8.)), Color::black());
    assert_eq!(color.color_at(&Point::new(1.36, -32., 12.)), Color::black());
    assert_eq!(color.color_at(&Point::new(0., 33.33, -5.7)), Color::black());
  }
}
