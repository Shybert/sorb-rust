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

  pub fn width(&self) -> usize {
    return self.width;
  }
  pub fn height(&self) -> usize {
    return self.height;
  }
  pub fn aspect_ratio(&self) -> f64 {
    return self.width() as f64 / self.height() as f64;
  }

  fn pixel_index(&self, x: usize, y: usize) -> usize {
    return y * self.width() + x;
  }
  pub fn pixel(&self, x: usize, y: usize) -> &Color {
    return &self.pixels[self.pixel_index(x, y)];
  }
  pub fn pixels(&self) -> &[Color] {
    return &self.pixels;
  }
  pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
    let index = self.pixel_index(x, y);
    self.pixels[index].set(color);
  }

  pub fn to_ppm(&self) -> String {
    let mut data = format!("P3\n{} {}\n255\n", self.width(), self.height());
    for pixel in self.pixels() {
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
        .pixels()
        .iter()
        .all(|pixel| pixel == &black),
      "Not all pixels were initialized to black"
    );
  }

  #[test]
  fn width() {
    assert_eq!(Canvas::new(10, 20).width(), 10);
  }

  #[test]
  fn height() {
    assert_eq!(Canvas::new(10, 20).height(), 20);
  }

  #[test]
  fn aspect_ratio() {
    assert_eq!(Canvas::new(48, 32).aspect_ratio(), 3. / 2.);
    assert_eq!(Canvas::new(32, 48).aspect_ratio(), 2. / 3.);
  }

  #[test]
  fn pixel() {
    assert_eq!(Canvas::new(10, 20).pixel(0, 0), &Color::black());
    assert_eq!(Canvas::new(10, 20).pixel(7, 15), &Color::black());
  }

  #[test]
  fn pixels() {
    assert_eq!(Canvas::new(10, 20).pixels().len(), 200);
  }

  #[test]
  fn get_set_pixel() {
    let mut canvas = Canvas::new(10, 20);
    let color = Color::new(-0.3, 0.45, 1.);
    canvas.set_pixel(3, 14, &color);
    assert_eq!(canvas.pixel(3, 14), &color);
  }
}
