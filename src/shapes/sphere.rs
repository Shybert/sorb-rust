use super::{Intersection, Shape};
use crate::geometry::{dot, Matrix, Ray};
use crate::utils::quadratic;

#[derive(Default)]
pub struct Sphere {
  transformation: Matrix,
}
impl Sphere {
  pub fn new(transformation: Matrix) -> Self {
    return Self { transformation };
  }
}
impl Shape for Sphere {
  fn get_transformation(&self) -> &Matrix {
    return &self.transformation;
  }
  fn set_transformation(&mut self, transformation: Matrix) {
    self.transformation = transformation;
  }

  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let transformed_ray = self.get_transformation().inverse() * *ray;
    let sphere_to_ray = transformed_ray.get_origin().into();
    let direction = transformed_ray.get_direction();

    let a = dot(direction, direction);
    let b = 2. * dot(direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.;

    let intersections = quadratic(a, b, c);
    return match intersections {
      Some((x1, x2)) => vec![Intersection::new(x1), Intersection::new(x2)],
      None => vec![],
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::geometry::{Point, Vector};

  #[test]
  fn init_new() {
    let scaling = Matrix::identity().scale(2., 2., 2.);
    let sphere = Sphere::new(scaling);
    assert_eq!(sphere.get_transformation(), &scaling);
  }

  #[test]
  fn init_default() {
    let sphere = Sphere::default();
    assert_eq!(sphere.get_transformation(), &Matrix::identity());
  }

  #[test]
  fn get_set_transformation() {
    let mut sphere = Sphere::default();
    let translation = Matrix::identity().translate(5., 4., 3.);
    sphere.set_transformation(translation);
    assert_eq!(sphere.get_transformation(), &translation);
  }

  #[test]
  fn intersection_ray_behind() {
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::default().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 4.);
    assert_eq!(intersections[1].time, 6.);
  }

  #[test]
  fn intersection_ray_at_tangent() {
    let ray = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::default().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 5.);
    assert_eq!(intersections[1].time, 5.);
  }

  #[test]
  fn intersection_ray_misses() {
    let ray = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::default().intersect(&ray);

    assert_eq!(intersections.len(), 0);
  }

  #[test]
  fn intersection_ray_inside() {
    let ray = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
    let intersections = Sphere::default().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -1.);
    assert_eq!(intersections[1].time, 1.);
  }

  #[test]
  fn intersection_ray_in_front() {
    let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::default().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -6.);
    assert_eq!(intersections[1].time, -4.);
  }

  #[test]
  fn intersection_scaled_sphere() {
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let mut sphere = Sphere::default();
    sphere.set_transformation(Matrix::identity().scale(2., 2., 2.));

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 3.);
    assert_eq!(intersections[1].time, 7.);
  }

  #[test]
  fn intersection_translated_sphere() {
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let mut sphere = Sphere::default();
    sphere.set_transformation(Matrix::identity().translate(5., 0., 0.));

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 0);
  }
}
