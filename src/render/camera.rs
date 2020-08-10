use super::{Canvas, Scene};
use crate::geometry::{Matrix, Point, Ray, Vector};
use crate::shapes::get_hit;

pub struct Camera {
  pub fov: f64,
}
impl Camera {
  pub fn render(&self, scene: &Scene, canvas: &mut Canvas, transform: Matrix) {
    let width = canvas.get_width() as f64;
    let height = canvas.get_height() as f64;

    let aspect_ratio = width / height;
    let computed_fov = (self.fov.to_radians() / 2.).tan();

    for i in 0..canvas.get_width() {
      for j in 0..canvas.get_height() {
        let x = (-1. + 2. * ((i as f64 + 0.5) / width)) * aspect_ratio * computed_fov;
        let y = (1. - 2. * ((j as f64 + 0.5) / height)) * computed_fov;
        let ray = transform * Ray::new(Point::new(0., 0., 0.), Vector::new(x, y, -1.));

        let intersections: Vec<_> = scene
          .get_objects()
          .iter()
          .flat_map(|object| object.intersect(&ray))
          .collect();
        let hit = get_hit(&intersections);

        if let Some(intersection) = hit {
          canvas.set_pixel(i, j, intersection.interaction.get_color());
        }
      }
    }
  }
}
