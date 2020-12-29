use crate::geometry::{Matrix, Point, Ray, Vector};
use crate::render::{lighting, Canvas, World};
use crate::shapes::find_hit;

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

  pub fn fov(&self) -> f64 {
    return self.fov;
  }

  pub fn camera_to_world(&self) -> &Matrix {
    return &self.camera_to_world;
  }

  pub fn render(&self, world: &World) -> Canvas {
    let mut canvas = Canvas::new(self.canvas_width(), self.canvas_height());

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let aspect_ratio = canvas.aspect_ratio();
    let computed_fov = (self.fov().to_radians() / 2.).tan();

    for i in 0..canvas.width() {
      for j in 0..canvas.height() {
        let x = (-1. + 2. * ((i as f64 + 0.5) / width)) * aspect_ratio * computed_fov;
        let y = (1. - 2. * ((j as f64 + 0.5) / height)) * computed_fov;
        let ray =
          *self.camera_to_world() * Ray::new(Point::new(0., 0., 0.), Vector::new(x, y, -1.));

        let intersections = world.intersect(&ray);
        let hit = find_hit(&intersections);

        if let Some(intersection) = hit {
          let color = lighting(
            intersection.material,
            ray.position(intersection.time),
            world.lights()[0],
            (ray.origin - ray.position(intersection.time)).normalize(),
            intersection.normal,
          );

          canvas.set_pixel(i, j, &color);
        }
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
}
