use super::{Intersection, Shape};
use crate::geometry::{dot, Ray};
use crate::utils::quadratic;

pub struct Sphere {}
impl Sphere {
  pub fn new() -> Self {
    return Self {};
  }
}
impl Shape for Sphere {
  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let sphere_to_ray = ray.get_origin().into();
    let a = dot(ray.get_direction(), ray.get_direction());
    let b = 2. * dot(ray.get_direction(), &sphere_to_ray);
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
  fn ray_starts_behind_sphere() {
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::new().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 4.);
    assert_eq!(intersections[1].time, 6.);
  }

  #[test]
  fn ray_intersects_at_tangent() {
    let ray = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::new().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 5.);
    assert_eq!(intersections[1].time, 5.);
  }

  #[test]
  fn ray_misses_sphere() {
    let ray = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::new().intersect(&ray);

    assert_eq!(intersections.len(), 0);
  }

  #[test]
  fn ray_starts_inside_sphere() {
    let ray = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
    let intersections = Sphere::new().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -1.);
    assert_eq!(intersections[1].time, 1.);
  }

  #[test]
  fn ray_starts_in_front_of_sphere() {
    let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::new().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -6.);
    assert_eq!(intersections[1].time, -4.);
  }
}
