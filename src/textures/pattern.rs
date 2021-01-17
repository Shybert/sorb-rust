use crate::geometry::{Matrix, Point};
use crate::textures::Texture;
use crate::utils::Lerp;
use crate::Color;
use std::fmt::Debug;

type FnPattern = fn(Point, Color, Color) -> Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern<A, B>
where
  A: Texture,
  B: Texture,
{
  fn_pattern: FnPattern,
  a: A,
  b: B,
  pattern_to_world: Matrix,
}
impl<A, B> Pattern<A, B>
where
  A: Texture,
  B: Texture,
{
  pub fn with_transformation(fn_pattern: FnPattern, a: A, b: B, pattern_to_world: Matrix) -> Self {
    return Self {
      fn_pattern,
      a,
      b,
      pattern_to_world,
    };
  }
  pub fn new(fn_pattern: FnPattern, a: A, b: B) -> Self {
    return Self::with_transformation(fn_pattern, a, b, Matrix::identity());
  }

  pub fn textures(&self) -> (&A, &B) {
    return (&self.a, &self.b);
  }
  pub fn pattern_to_world(&self) -> &Matrix {
    return &self.pattern_to_world;
  }
}
impl Pattern<Color, Color> {
  pub fn with_fn(fn_pattern: FnPattern) -> Self {
    return Self::with_transformation(
      fn_pattern,
      Color::white(),
      Color::black(),
      Matrix::identity(),
    );
  }
}
impl<A, B> Texture for Pattern<A, B>
where
  A: Texture,
  B: Texture,
{
  fn color_at(&self, point: &Point) -> Color {
    let point_pattern = self.pattern_to_world().inverse() * *point;
    return (self.fn_pattern)(
      point_pattern,
      self.a.color_at(&point_pattern),
      self.b.color_at(&point_pattern),
    );
  }
}

pub fn stripes(point: Point, a: Color, b: Color) -> Color {
  return if point.x.rem_euclid(2.).floor() == 0. {
    a
  } else {
    b
  };
}

pub fn gradient(point: Point, a: Color, b: Color) -> Color {
  if point.x.abs().rem_euclid(2.).floor() == 0. {
    return a.lerp(&b, &point.x.abs().fract());
  } else {
    return b.lerp(&a, &point.x.abs().fract());
  }
}

pub fn ring(point: Point, a: Color, b: Color) -> Color {
  let radius = (point.x.powi(2) + point.z.powi(2)).sqrt();
  if radius.rem_euclid(2.).floor() == 0. {
    return a;
  } else {
    return b;
  }
}

pub fn checkers(point: Point, a: Color, b: Color) -> Color {
  if (point.x.floor() + point.y.floor() + point.z.floor()).rem_euclid(2.) == 0. {
    return a;
  } else {
    return b;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_fn_pattern(point: Point, a: Color, b: Color) -> Color {
    if point.x >= 0. {
      return a;
    } else {
      return b;
    }
  }

  #[test]
  fn init_with_transformation() {
    let pattern = Pattern::with_transformation(
      test_fn_pattern,
      Color::white(),
      Color::black(),
      Matrix::identity().scale(5., -5., 5.),
    );
    assert_eq!(pattern.textures(), (&Color::white(), &Color::black()));
    assert_eq!(
      pattern.pattern_to_world(),
      &Matrix::identity().scale(5., -5., 5.)
    );
  }

  #[test]
  fn init_with_func() {
    let pattern = Pattern::with_fn(test_fn_pattern);
    assert_eq!(pattern.textures(), (&Color::white(), &Color::black()));
    assert_eq!(pattern.pattern_to_world(), &Matrix::identity());
  }

  #[test]
  fn init_new() {
    let pattern = Pattern::new(test_fn_pattern, Color::cyan(), Color::red());
    assert_eq!(pattern.textures(), (&Color::cyan(), &Color::red()));
    assert_eq!(pattern.pattern_to_world(), &Matrix::identity());
  }

  #[test]
  fn equality_identical() {
    let color_a = Color::new(0.3, 0.3, 0.6);
    let color_b = Color::new(0.6, 0.2, 0.1);
    let a = Pattern::new(test_fn_pattern, color_a, color_b);
    let b = Pattern::new(test_fn_pattern, color_a, color_b);

    assert_eq!(a, b);

    let b = Pattern::new(test_fn_pattern, color_a, Color::new(0.6000001, 0.2, 0.1));
    assert_eq!(a, b);
  }

  #[test]
  fn equality_different() {
    let a = Pattern::new(test_fn_pattern, Color::yellow(), Color::cyan());
    let b = Pattern::new(test_fn_pattern, Color::red(), Color::green());
    assert_ne!(a, b);
  }

  #[test]
  fn color_at() {
    let pattern = Pattern::with_fn(test_fn_pattern);

    assert_eq!(pattern.color_at(&Point::origin()), Color::white());
    assert_eq!(pattern.color_at(&Point::new(-1., 0., 0.)), Color::black());
    assert_eq!(pattern.color_at(&Point::new(1., 0., 0.)), Color::white());

    assert_eq!(pattern.color_at(&Point::new(0., -1., 0.)), Color::white());
    assert_eq!(pattern.color_at(&Point::new(0., 1., 0.)), Color::white());

    assert_eq!(pattern.color_at(&Point::new(0., 0., -1.)), Color::white());
    assert_eq!(pattern.color_at(&Point::new(0., 0., 1.)), Color::white());
  }

  #[test]
  fn color_at_with_transformation() {
    let pattern = Pattern::with_transformation(
      test_fn_pattern,
      Color::white(),
      Color::black(),
      Matrix::identity().translate(5., 0., 0.),
    );

    assert_eq!(pattern.color_at(&Point::new(5., 0., 0.)), Color::white());
    assert_eq!(pattern.color_at(&Point::new(4., 0., 0.)), Color::black());
    assert_eq!(pattern.color_at(&Point::new(6., 0., 0.)), Color::white());

    assert_eq!(pattern.color_at(&Point::new(5., -1., 0.)), Color::white());
    assert_eq!(pattern.color_at(&Point::new(5., 1., 0.)), Color::white());

    assert_eq!(pattern.color_at(&Point::new(5., 0., -1.)), Color::white());
    assert_eq!(pattern.color_at(&Point::new(5., 0., 1.)), Color::white());
  }

  fn stripes_default(point: Point) -> Color {
    return stripes(point, Color::white(), Color::black());
  }
  #[test]
  fn stripes_alternates_in_x() {
    assert_eq!(stripes_default(Point::origin()), Color::white());
    assert_eq!(stripes_default(Point::new(0.9, 0., 0.)), Color::white());
    assert_eq!(stripes_default(Point::new(1., 0., 0.)), Color::black());
    assert_eq!(stripes_default(Point::new(-0.1, 0., 0.)), Color::black());
    assert_eq!(stripes_default(Point::new(-1., 0., 0.)), Color::black());
    assert_eq!(stripes_default(Point::new(-1.1, 0., 0.)), Color::white());
  }
  #[test]
  fn stripes_constant_in_y() {
    assert_eq!(stripes_default(Point::origin()), Color::white());
    assert_eq!(stripes_default(Point::new(0., 0.9, 0.)), Color::white());
    assert_eq!(stripes_default(Point::new(0., 1., 0.)), Color::white());
    assert_eq!(stripes_default(Point::new(0., -0.1, 0.)), Color::white());
    assert_eq!(stripes_default(Point::new(0., -1., 0.)), Color::white());
    assert_eq!(stripes_default(Point::new(0., -1.1, 0.)), Color::white());
  }

  #[test]
  fn stripes_constant_in_z() {
    assert_eq!(stripes_default(Point::origin()), Color::white());
    assert_eq!(stripes_default(Point::new(0., 0., 0.9)), Color::white());
    assert_eq!(stripes_default(Point::new(0., 0., 1.)), Color::white());
    assert_eq!(stripes_default(Point::new(0., 0., -0.1)), Color::white());
    assert_eq!(stripes_default(Point::new(0., 0., -1.)), Color::white());
    assert_eq!(stripes_default(Point::new(0., 0., -1.1)), Color::white());
  }

  fn gradient_default(point: Point) -> Color {
    return gradient(point, Color::white(), Color::black());
  }
  #[test]
  fn gradient_linearly_interpolates_in_x() {
    assert_eq!(gradient_default(Point::origin()), Color::white());
    assert_eq!(
      gradient_default(Point::new(0.25, 0., 0.)),
      Color::new(0.75, 0.75, 0.75)
    );
    assert_eq!(
      gradient_default(Point::new(0.5, 0., 0.)),
      Color::new(0.5, 0.5, 0.5)
    );
    assert_eq!(
      gradient_default(Point::new(0.75, 0., 0.)),
      Color::new(0.25, 0.25, 0.25)
    );
  }
  #[test]
  fn gradient_is_continuous() {
    assert_eq!(
      gradient_default(Point::new(1.25, 1.25, 1.25)),
      Color::new(0.25, 0.25, 0.25)
    );
    assert_eq!(
      gradient_default(Point::new(0.75, 0., 0.)),
      Color::new(0.25, 0.25, 0.25)
    );

    assert_eq!(
      gradient_default(Point::new(0.25, 0., 0.)),
      Color::new(0.75, 0.75, 0.75)
    );
    assert_eq!(
      gradient_default(Point::new(-0.25, 0., 0.)),
      Color::new(0.75, 0.75, 0.75)
    );
  }
  #[test]
  fn gradient_constant_in_y() {
    assert_eq!(gradient_default(Point::origin()), Color::white());
    assert_eq!(gradient_default(Point::new(0., 0.9, 0.)), Color::white());
    assert_eq!(gradient_default(Point::new(0., 1., 0.)), Color::white());
    assert_eq!(gradient_default(Point::new(0., -0.1, 0.)), Color::white());
    assert_eq!(gradient_default(Point::new(0., -1., 0.)), Color::white());
    assert_eq!(gradient_default(Point::new(0., -1.1, 0.)), Color::white());
  }
  #[test]
  fn gradient_constant_in_z() {
    assert_eq!(gradient_default(Point::origin()), Color::white());
    assert_eq!(gradient_default(Point::new(0., 0., 0.9)), Color::white());
    assert_eq!(gradient_default(Point::new(0., 0., 1.)), Color::white());
    assert_eq!(gradient_default(Point::new(0., 0., -0.1)), Color::white());
    assert_eq!(gradient_default(Point::new(0., 0., -1.)), Color::white());
    assert_eq!(gradient_default(Point::new(0., 0., -1.1)), Color::white());
  }

  fn ring_default(point: Point) -> Color {
    return ring(point, Color::white(), Color::black());
  }
  #[test]
  fn ring_alternates_in_x_and_z() {
    assert_eq!(ring_default(Point::origin()), Color::white());
    assert_eq!(ring_default(Point::new(-1.5, 0., 0.)), Color::black());
    assert_eq!(ring_default(Point::new(1.5, 0., 0.)), Color::black());
    assert_eq!(ring_default(Point::new(-2.5, 0., 0.)), Color::white());
    assert_eq!(ring_default(Point::new(2.5, 0., 0.)), Color::white());

    assert_eq!(ring_default(Point::new(0., 0., -1.5)), Color::black());
    assert_eq!(ring_default(Point::new(0., 0., 1.5)), Color::black());
    assert_eq!(ring_default(Point::new(0., 0., -2.5)), Color::white());
    assert_eq!(ring_default(Point::new(0., 0., 2.5)), Color::white());

    assert_eq!(ring_default(Point::new(0.708, 0., 0.708)), Color::black());
    assert_eq!(ring_default(Point::new(-0.708, 0., -0.708)), Color::black());
    assert_eq!(ring_default(Point::new(1.415, 0., 1.415)), Color::white());
    assert_eq!(ring_default(Point::new(-1.415, 0., -1.415)), Color::white());
  }

  #[test]
  fn ring_constant_in_y() {
    assert_eq!(ring_default(Point::new(0., -1.5, 0.,)), Color::white());
    assert_eq!(ring_default(Point::new(0., 1.5, 0.,)), Color::white());
    assert_eq!(ring_default(Point::new(0., -2.5, 0.,)), Color::white());
    assert_eq!(ring_default(Point::new(0., 2.5, 0.,)), Color::white());
  }

  fn checkers_default(point: Point) -> Color {
    return checkers(point, Color::white(), Color::black());
  }
  #[test]
  fn checkers_alternates_in_x() {
    assert_eq!(checkers_default(Point::new(-0.5, 0., 0.)), Color::black());
    assert_eq!(checkers_default(Point::new(0.5, 0., 0.)), Color::white());
    assert_eq!(checkers_default(Point::new(1.5, 0., 0.)), Color::black());
  }
  #[test]
  fn checkers_alternates_in_y() {
    assert_eq!(checkers_default(Point::new(0., -0.5, 0.)), Color::black());
    assert_eq!(checkers_default(Point::new(0., 0.5, 0.)), Color::white());
    assert_eq!(checkers_default(Point::new(0., 1.5, 0.)), Color::black());
  }
  #[test]
  fn checkers_alternates_in_z() {
    assert_eq!(checkers_default(Point::new(0., 0., -0.5)), Color::black());
    assert_eq!(checkers_default(Point::new(0., 0., 0.5)), Color::white());
    assert_eq!(checkers_default(Point::new(0., 0., 1.5)), Color::black());
  }
}
