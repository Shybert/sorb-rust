use super::utils;

#[derive(Debug, Clone)]
struct Color {
  r: f64,
  g: f64,
  b: f64,
}
impl Color {
  fn new(r: f64, g: f64, b: f64) -> Self {
    return Color { r, g, b };
  }
}

impl PartialEq<Self> for Color {
  fn eq(&self, other: &Self) -> bool {
    return utils::approx_equals(self.r, other.r)
      && utils::approx_equals(self.g, other.g)
      && utils::approx_equals(self.b, other.b);
  }
}

struct Canvas {
  width: usize,
  height: usize,
  pixels: Vec<Color>,
}
impl Canvas {
  fn new(width: usize, height: usize) -> Self {
    return Canvas {
      width,
      height,
      pixels: vec![Color::new(0., 0., 0.); width * height],
    };
  }

  fn get_width(&self) -> usize {
    return self.width;
  }

  fn get_height(&self) -> usize {
    return self.height;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color_equality() {
    assert_eq!(Color::new(4., -4., 3.), Color::new(4., -4., 3.));

    assert_eq!(Color::new(1.000000001, 0., 0.), Color::new(1., 0., 0.));
  }

  #[test]
  fn canvas_get_width() {
    assert_eq!(Canvas::new(10, 20).get_width(), 10);
  }

  #[test]
  fn canvas_get_height() {
    assert_eq!(Canvas::new(10, 20).get_height(), 20);
  }

  // #[test]
  // fn canvas_get_pixel() {
  //   assert_eq!(Canvas::new(10, 20).get_pixel(0, 0), Color::new(0., 0., 0.));
  // }
}
