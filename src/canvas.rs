use crate::utils::{approx_equals, clamp_number};

#[derive(Clone, Copy, Debug)]
pub struct Color {
  r: f64,
  g: f64,
  b: f64,
}
impl Color {
  pub fn new(r: f64, g: f64, b: f64) -> Self {
    return Self { r, g, b };
  }

  pub fn set(&mut self, new: &Self) {
    self.r = new.r;
    self.g = new.g;
    self.b = new.b;
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    return approx_equals(self.r, other.r)
      && approx_equals(self.g, other.g)
      && approx_equals(self.b, other.b);
  }
}

#[derive(Clone)]
pub struct Canvas {
  width: usize,
  height: usize,
  pixels: Vec<Color>,
}
impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    return Self {
      width,
      height,
      pixels: vec![Color::new(0., 0., 0.); width * height],
    };
  }

  pub fn get_width(&self) -> usize {
    return self.width;
  }

  pub fn get_height(&self) -> usize {
    return self.height;
  }

  fn get_pixel_index(&self, x: usize, y: usize) -> usize {
    return y * self.get_width() + x;
  }
  pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
    return &self.pixels[self.get_pixel_index(x, y)];
  }
  pub fn get_pixels(&self) -> &[Color] {
    return &self.pixels[..];
  }
  pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
    let index = self.get_pixel_index(x, y);
    self.pixels[index].set(color);
  }

  pub fn to_ppm(&self) -> String {
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
  use crate::assert_ae;

  #[test]
  fn color_init() {
    let color = Color::new(0.5, 0.7, 0.12443);
    assert_ae!(color.r, 0.5);
    assert_ae!(color.g, 0.7);
    assert_ae!(color.b, 0.12443);
  }

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
