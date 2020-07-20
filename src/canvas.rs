use crate::utils::{approx_equals, clamp_number};

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

  fn set(&mut self, new: &Color) {
    self.r = new.r;
    self.g = new.g;
    self.b = new.b;
  }
}

impl PartialEq<Self> for Color {
  fn eq(&self, other: &Self) -> bool {
    return approx_equals(&self.r, &other.r)
      && approx_equals(&self.g, &other.g)
      && approx_equals(&self.b, &other.b);
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

  fn get_pixel_index(&self, x: usize, y: usize) -> usize {
    return y * self.get_width() + x;
  }
  fn get_pixel(&self, x: usize, y: usize) -> &Color {
    return &self.pixels[self.get_pixel_index(x, y)];
  }
  fn get_pixels(&self) -> &[Color] {
    return &self.pixels[..];
  }
  fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
    let index = self.get_pixel_index(x, y);
    self.pixels[index].set(color);
  }

  fn to_ppm(&self) -> String {
    let mut data = format!("P3\n{} {}\n255\n", self.get_width(), self.get_height());
    for pixel in self.get_pixels() {
      data.push_str(&format!(
        "{} {} {} ",
        clamp_number(pixel.r * 255., 0., 255.).round(),
        clamp_number(pixel.g * 255., 0., 255.).round(),
        clamp_number(pixel.b * 255., 0., 255.).round()
      ));
    }

    return data;
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
  fn color_set() {
    let mut old = Color::new(0., 0., 0.);
    let new = Color::new(-0.3, 0.45, 1.);
    old.set(&new);
    assert_eq!(old, new);
  }

  #[test]
  fn canvas_init() {
    let black = Color::new(0., 0., 0.);
    assert!(
      Canvas::new(10, 20)
        .get_pixels()
        .iter()
        .all(|pixel| pixel == &black),
      "Not all pixels were initialized to black"
    );
  }

  #[test]
  fn canvas_get_width() {
    assert_eq!(Canvas::new(10, 20).get_width(), 10);
  }

  #[test]
  fn canvas_get_height() {
    assert_eq!(Canvas::new(10, 20).get_height(), 20);
  }

  #[test]
  fn canvas_get_pixel() {
    assert_eq!(Canvas::new(10, 20).get_pixel(0, 0), &Color::new(0., 0., 0.));
    assert_eq!(
      Canvas::new(10, 20).get_pixel(7, 15),
      &Color::new(0., 0., 0.)
    );
  }

  #[test]
  fn canvas_get_pixels() {
    assert_eq!(Canvas::new(10, 20).get_pixels().len(), 200);
  }

  #[test]
  fn canvas_set_pixel() {
    let mut canvas = Canvas::new(10, 20);
    let color = Color::new(-0.3, 0.45, 1.);
    canvas.set_pixel(3, 14, &color);
    assert_eq!(canvas.get_pixel(3, 14), &color);
  }
}
