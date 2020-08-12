use crate::color::Color;
use crate::utils::clamp_number;

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
      pixels: vec![Color::black(); width * height],
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

  #[test]
  fn init() {
    let black = Color::black();
    assert!(
      Canvas::new(10, 20)
        .get_pixels()
        .iter()
        .all(|pixel| pixel == &black),
      "Not all pixels were initialized to black"
    );
  }

  #[test]
  fn get_width() {
    assert_eq!(Canvas::new(10, 20).get_width(), 10);
  }

  #[test]
  fn get_height() {
    assert_eq!(Canvas::new(10, 20).get_height(), 20);
  }

  #[test]
  fn get_pixel() {
    assert_eq!(Canvas::new(10, 20).get_pixel(0, 0), &Color::black());
    assert_eq!(Canvas::new(10, 20).get_pixel(7, 15), &Color::black());
  }

  #[test]
  fn get_pixels() {
    assert_eq!(Canvas::new(10, 20).get_pixels().len(), 200);
  }

  #[test]
  fn set_pixel() {
    let mut canvas = Canvas::new(10, 20);
    let color = Color::new(-0.3, 0.45, 1.);
    canvas.set_pixel(3, 14, &color);
    assert_eq!(canvas.get_pixel(3, 14), &color);
  }
}
