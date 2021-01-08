use crate::geometry::Point;
use crate::Color;
use std::fmt::Debug;

mod stripes;
pub use stripes::*;

pub trait Pattern: Debug {
  fn get_colors(&self) -> Vec<&Color>;

  fn color_at(&self, point: &Point) -> Color;
}
