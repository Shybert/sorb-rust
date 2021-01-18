use super::Shape;
use crate::geometry::{Material, Matrix, Point, Ray, Vector};
use crate::utils::quadratic;

#[derive(Default)]
pub struct Sphere {
  material: Material,
  object_to_world: Matrix,
}
impl Sphere {
  pub fn new(material: Material, object_to_world: Matrix) -> Self {
    return Self {
      material,
      object_to_world,
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

  fn object_to_world(&self) -> &Matrix {
    return &self.object_to_world;
  }
  fn set_object_to_world(&mut self, object_to_world: Matrix) {
    self.object_to_world = object_to_world;
  }

  fn intersect_object_space(&self, ray: &Ray) -> Vec<f64> {
    let sphere_to_ray = ray.origin - Point::origin();
    let direction = ray.direction;

    let a = direction.dot(&direction);
    let b = 2. * direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

    let intersection_times = quadratic(a, b, c);
    return match intersection_times {
      Some((t1, t2)) => vec![t1, t2],
      None => vec![],
    };
  }

  fn normal_at_object_space(&self, point: &Point) -> Vector {
    return *point - Point::origin();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::geometry::{Point, Vector};
  use crate::Color;
  use std::f64::consts::{PI, SQRT_2};

  #[test]
  fn init_new() {
    let material = Material::new(Box::new(Color::yellow()), 0.3, 0.3, 0.3, 70.);
    let scaling = Matrix::identity().scale(2., 2., 2.);
    let sphere = Sphere::new(material, scaling);
    assert_eq!(
      sphere.material().color_at(&Point::origin()),
      Color::yellow()
    );
    assert_eq!(sphere.object_to_world(), &scaling);
  }

  #[test]
  fn init_default() {
    let sphere = Sphere::default();
    assert_eq!(
      sphere.material().color_at(&Point::origin()),
      Material::default().color_at(&Point::origin())
    );
    assert_eq!(sphere.object_to_world(), &Matrix::identity());
  }

  #[test]
  fn get_set_material() {
    let mut sphere = Sphere::default();
    let material = Material::new(Box::new(Color::cyan()), 0.1, 0.4, 0.5, 50.);
    sphere.set_material(material);
    assert_eq!(sphere.material().color_at(&Point::origin()), Color::cyan());
  }

  #[test]
  fn get_set_object_to_world() {
    let mut sphere = Sphere::default();
    let translation = Matrix::identity().translate(5., 4., 3.);
    sphere.set_object_to_world(translation);
    assert_eq!(sphere.object_to_world(), &translation);
  }

  #[test]
  fn intersection_has_intersection_point() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].point, Point::new(0., 0., -1.));
    assert_eq!(intersections[1].point, Point::new(0., 0., 1.));
  }

  #[test]
  fn intersection_has_outgoing_vector() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].outgoing, Vector::new(0., 0., -1.));
    assert_eq!(intersections[1].outgoing, Vector::new(0., 0., -1.));
  }

  #[test]
  fn intersection_outgoing_vector_is_normalized() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(
      intersections[0].outgoing,
      intersections[0].outgoing.normalize()
    );
    assert_eq!(
      intersections[1].outgoing,
      intersections[1].outgoing.normalize()
    );
  }

  #[test]
  fn intersection_has_sphere_material() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(
      intersections[0].material.color_at(&Point::origin()),
      sphere.material().color_at(&Point::origin())
    );
    assert_eq!(
      intersections[1].material.color_at(&Point::origin()),
      sphere.material().color_at(&Point::origin())
    );
  }

  #[test]
  fn intersection_has_normal_at_intersection() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].normal, Vector::new(0., 0., -1.));
    assert_eq!(intersections[1].normal, Vector::new(0., 0., 1.));
  }

  #[test]
  fn intersection_ray_behind() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 4.);
    assert_eq!(intersections[1].time, 6.);
  }

  #[test]
  fn intersection_ray_at_tangent() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 5.);
    assert_eq!(intersections[1].time, 5.);
  }

  #[test]
  fn intersection_ray_misses() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 0);
  }

  #[test]
  fn intersection_ray_inside() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::origin(), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -1.);
    assert_eq!(intersections[1].time, 1.);
  }

  #[test]
  fn intersection_ray_in_front() {
    let sphere = Sphere::default();
    let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, -6.);
    assert_eq!(intersections[1].time, -4.);
  }

  #[test]
  fn intersection_scaled_sphere() {
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let mut sphere = Sphere::default();
    sphere.set_object_to_world(Matrix::identity().scale(2., 2., 2.));

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].time, 3.);
    assert_eq!(intersections[1].time, 7.);
  }

  #[test]
  fn intersection_translated_sphere() {
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let mut sphere = Sphere::default();
    sphere.set_object_to_world(Matrix::identity().translate(5., 0., 0.));

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
    sphere.set_object_to_world(Matrix::identity().translate(0., 1., 0.));
    let normal = sphere.normal_at(&Point::new(0., (SQRT_2 / 2.) + 1., SQRT_2 / 2.));
    assert_eq!(normal, Vector::new(0., SQRT_2 / 2., SQRT_2 / 2.));
  }

  #[test]
  fn normal_of_transformed_sphere() {
    let mut sphere = Sphere::default();
    sphere.set_object_to_world(Matrix::identity().rotate_z(PI / 5.).scale(1., 0.5, 1.));
    let normal = sphere.normal_at(&Point::new(0., SQRT_2 / 2., -(SQRT_2 / 2.)));
    assert_eq!(normal, Vector::new(0., 0.97014, -0.24254))
  }
}
