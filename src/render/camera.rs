use crate::geometry::{Matrix, Point, Ray, Vector};
use crate::render::{Canvas, World};

pub struct Camera {
  canvas_width: usize,
  canvas_height: usize,
  fov: f64,
  camera_to_world: Matrix,
}
impl Camera {
  pub fn new(canvas_width: usize, canvas_height: usize, fov: f64, camera_to_world: Matrix) -> Self {
    return Self {
      canvas_width,
      canvas_height,
      fov,
      camera_to_world,
    };
  }

  pub fn canvas_width(&self) -> usize {
    return self.canvas_width;
  }
  pub fn canvas_height(&self) -> usize {
    return self.canvas_height;
  }
  pub fn aspect_ratio(&self) -> f64 {
    return self.canvas_width() as f64 / self.canvas_height() as f64;
  }

  pub fn fov(&self) -> f64 {
    return self.fov;
  }
  fn fov_scale(&self) -> f64 {
    return (self.fov().to_radians() / 2.).tan();
  }

  pub fn camera_to_world(&self) -> &Matrix {
    return &self.camera_to_world;
  }

  fn pixel_size(&self) -> f64 {
    return self.fov_scale() * 2. / self.canvas_height() as f64;
  }
  fn canvas_left_edge(&self) -> f64 {
    return -1. * self.aspect_ratio() * self.fov_scale();
  }
  fn canvas_top_edge(&self) -> f64 {
    return 1. * self.fov_scale();
  }
  pub fn ray_for_pixel(&self, x_pixel: usize, y_pixel: usize) -> Ray {
    let x_offset = (x_pixel as f64 + 0.5) * self.pixel_size();
    let y_offset = (y_pixel as f64 + 0.5) * self.pixel_size();

    let x_world = self.canvas_left_edge() + x_offset;
    let y_world = self.canvas_top_edge() - y_offset;

    let direction = Vector::new(x_world, y_world, -1.).normalize();
    return *self.camera_to_world() * Ray::new(Point::origin(), direction);
  }

  pub fn render(&self, world: &World) -> Canvas {
    let mut canvas = Canvas::new(self.canvas_width(), self.canvas_height());

    for i in 0..canvas.width() {
      for j in 0..canvas.height() {
        let ray = self.ray_for_pixel(i, j);
        let color = world.color_at(&ray);
        canvas.set_pixel(i, j, &color);
      }
    }

    return canvas;
  }
}
impl Default for Camera {
  fn default() -> Self {
    return Camera::new(640, 640, 90., Matrix::identity());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    let camera = Camera::new(400, 320, 90., Matrix::identity().scale(-1., 1., -1.));
    assert_eq!(camera.canvas_width(), 400);
    assert_eq!(camera.canvas_height(), 320);
    assert_eq!(camera.fov(), 90.);
    assert_eq!(
      camera.camera_to_world(),
      &Matrix::identity().scale(-1., 1., -1.)
    );
  }

  #[test]
  fn init_default() {
    let camera = Camera::default();
    assert_eq!(camera.canvas_width(), 640);
    assert_eq!(camera.canvas_height(), 640);
    assert_eq!(camera.fov(), 90.);
    assert_eq!(camera.camera_to_world(), &Matrix::identity());
  }

  #[test]
  fn aspect_ratio() {
    assert_eq!(
      Camera::new(48, 32, 90., Matrix::identity()).aspect_ratio(),
      3. / 2.
    );
    assert_eq!(
      Camera::new(32, 48, 90., Matrix::identity()).aspect_ratio(),
      2. / 3.
    );
  }

  #[test]
  fn ray_for_pixel_canvas_center() {
    let camera = Camera::new(201, 101, 90., Matrix::identity());
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray, Ray::new(Point::origin(), Vector::new(0., 0., -1.)));
  }

  #[test]
  fn ray_for_pixel_canvas_corner() {
    let camera = Camera::new(201, 101, 90., Matrix::identity());
    let ray = camera.ray_for_pixel(0, 0);
    assert_eq!(
      ray,
      Ray::new(Point::origin(), Vector::new(-0.815132, 0.407566, -0.411642))
    );
  }

  #[test]
  fn ray_for_pixel_transformed_camera() {
    let camera = Camera::new(
      201,
      101,
      90.,
      Matrix::look_at(
        &Point::new(0., 2., -5.),
        &Point::new(5., -6., 1.),
        &Vector::new(1., 1., 0.),
      ),
    );
    let ray = camera.ray_for_pixel(60, 30);
    assert_eq!(
      ray,
      Ray::new(
        Point::new(0., 2., -5.),
        Vector::new(0.794633, -0.591306, -0.055593)
      )
    );
  }

  #[test]
  fn ray_for_pixel_small_fov() {
    let camera = Camera::new(201, 101, 30., Matrix::identity());
    let ray = camera.ray_for_pixel(60, 30);
    assert_eq!(
      ray,
      Ray::new(Point::origin(), Vector::new(-0.206503, 0.103251, -0.972983))
    );
  }

  #[test]
  fn ray_for_pixel_large_fov() {
    let camera = Camera::new(201, 101, 120., Matrix::identity());
    let ray = camera.ray_for_pixel(60, 30);
    assert_eq!(
      ray,
      Ray::new(Point::origin(), Vector::new(-0.749258, 0.374629, -0.546137))
    );
  }

  #[test]
  fn ray_for_pixel_large_fov_transformed_camera() {
    let camera = Camera::new(
      201,
      101,
      120.,
      Matrix::look_at(
        &Point::new(0., 2., -5.),
        &Point::new(5., -6., 1.),
        &Vector::new(1., 1., 0.),
      ),
    );
    let ray = camera.ray_for_pixel(60, 30);
    assert_eq!(
      ray,
      Ray::new(
        Point::new(0., 2., -5.),
        Vector::new(0.825254, -0.461065, -0.284800)
      )
    );
  }
}
