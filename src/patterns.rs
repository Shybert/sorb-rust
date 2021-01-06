use crate::geometry::Point;
use crate::Color;

mod stripes;
pub use stripes::*;

trait Pattern {
  fn get_colors(&self) -> Vec<&Color>;

  fn color_at(&self, point: &Point) -> Color;
}
