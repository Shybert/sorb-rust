use super::{Intersection, Shape};
use crate::geometry::{dot, Material, Matrix, Point, Ray, Vector};
use crate::utils::quadratic;

#[derive(Default)]
pub struct Sphere {
  material: Material,
  transformation: Matrix,
}
impl Sphere {
  pub fn new(material: Material, transformation: Matrix) -> Self {
    return Self {
      material,
      transformation,
    };
  }
}
impl Shape for Sphere {
  fn material(&self) -> &Material {
    return &self.material;
  }
  fn set_material(&mut self, material: Material) {
    self.material = material;
  }

  fn transformation(&self) -> &Matrix {
    return &self.transformation;
  }
  fn set_transformation(&mut self, transformation: Matrix) {
    self.transformation = transformation;
  }

  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let object_ray = self.transformation().inverse() * *ray;
    let sphere_to_ray = *object_ray.origin() - Point::new(0., 0., 0.);
    let direction = object_ray.direction();

    let a = dot(direction, direction);
    let b = 2. * dot(direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.;

    let intersections = quadratic(a, b, c);
    return match intersections {
      Some((x1, x2)) => vec![
        Intersection::new(x1, *self.material()),
        Intersection::new(x2, *self.material()),
      ],
      None => vec![],
    };
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_point = self.transformation().inverse() * *point;
    let object_normal = object_point - Point::new(0., 0., 0.);
    let world_normal = self.transformation().inverse().transpose() * object_normal;
    return world_normal.normalize();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::geometry::{Point, Vector};
  use crate::Color;
  use std::f64::consts::PI;

  #[test]
  fn init_new() {
    let material = Material::new(Color::yellow(), 0.3, 0.3, 0.3, 70.);
    let scaling = Matrix::identity().scale(2., 2., 2.);
    let sphere = Sphere::new(material, scaling);
    assert_eq!(sphere.material(), &material);
    assert_eq!(sphere.transformation(), &scaling);
  }

  #[test]
  fn init_default() {
    let sphere = Sphere::default();
    assert_eq!(sphere.material(), &Material::default());
    assert_eq!(sphere.transformation(), &Matrix::identity());
  }

  #[test]
  fn get_set_material() {
    let mut sphere = Sphere::default();
    let material = Material::new(Color::cyan(), 0.1, 0.4, 0.5, 50.);
    sphere.set_material(material);
    assert_eq!(sphere.material(), &material);
  }

  #[test]
  fn get_set_transformation() {
    let mut sphere = Sphere::default();
    let translation = Matrix::identity().translate(5., 4., 3.);
    sphere.set_transformation(translation);
    assert_eq!(sphere.transformation(), &translation);
  }

  #[test]
  fn intersect_interaction_has_sphere_material() {
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = Sphere::default().intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].material, Material::default());
    assert_eq!(intersections[1].material, Material::default());
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

  #[test]
  fn normal_at_x_axis() {
    let sphere = Sphere::default();
    let normal = sphere.normal_at(&Point::new(1., 0., 0.));
    assert_eq!(normal, Vector::new(1., 0., 0.));
  }

  #[test]
  fn normal_at_y_axis() {
    let sphere = Sphere::default();
    let normal = sphere.normal_at(&Point::new(0., 1., 0.));
    assert_eq!(normal, Vector::new(0., 1., 0.));
  }

  #[test]
  fn normal_at_z_axis() {
    let sphere = Sphere::default();
    let normal = sphere.normal_at(&Point::new(0., 0., 1.));
    assert_eq!(normal, Vector::new(0., 0., 1.));
  }

  #[test]
  fn normal_at_non_axial_point() {
    let sphere = Sphere::default();
    let normal = sphere.normal_at(&Point::new(
      3_f64.sqrt() / 3.,
      3_f64.sqrt() / 3.,
      3_f64.sqrt() / 3.,
    ));
    assert_eq!(
      normal,
      Vector::new(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)
    );
  }

  #[test]
  fn normal_is_normalized() {
    let sphere = Sphere::default();
    let normal = sphere.normal_at(&Point::new(
      3_f64.sqrt() / 3.,
      3_f64.sqrt() / 3.,
      3_f64.sqrt() / 3.,
    ));
    assert_eq!(normal.normalize(), normal);
  }

  #[test]
  fn normal_of_translated_sphere() {
    let mut sphere = Sphere::default();
    sphere.set_transformation(Matrix::identity().translate(0., 1., 0.));
    let normal = sphere.normal_at(&Point::new(0., (2_f64.sqrt() / 2.) + 1., 2_f64.sqrt() / 2.));
    assert_eq!(
      normal,
      Vector::new(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
    );
  }

  #[test]
  fn normal_of_transformed_sphere() {
    let mut sphere = Sphere::default();
    sphere.set_transformation(Matrix::identity().rotate_z(PI / 5.).scale(1., 0.5, 1.));
    let normal = sphere.normal_at(&Point::new(0., 2_f64.sqrt() / 2., -(2_f64.sqrt() / 2.)));
    assert_eq!(normal, Vector::new(0., 0.97014, -0.24254))
  }
}
