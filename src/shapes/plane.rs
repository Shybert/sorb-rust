use super::Shape;
use crate::geometry::{Material, Matrix, Point, Ray, Vector};

#[derive(Default)]
pub struct Plane {
  material: Material,
  transformation: Matrix,
}
impl Plane {
  pub fn new(material: Material, transformation: Matrix) -> Self {
    return Self {
      material,
      transformation,
    };
  }
}
impl Shape for Plane {
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

  fn intersect_object_space(&self, ray: &Ray) -> Vec<f64> {
    todo!();
  }

  fn normal_at_object_space(&self, _point: &Point) -> Vector {
    return Vector::new(0., 1., 0.);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Color;
  use std::f64::consts::PI;

  #[test]
  fn init_new() {
    let material = Material::new(Color::yellow(), 0.3, 0.3, 0.3, 70.);
    let scaling = Matrix::identity().scale(2., 2., 2.);
    let plane = Plane::new(material, scaling);
    assert_eq!(plane.material(), &material);
    assert_eq!(plane.transformation(), &scaling);
  }

  #[test]
  fn init_default() {
    let plane = Plane::default();
    assert_eq!(plane.material(), &Material::default());
    assert_eq!(plane.transformation(), &Matrix::identity());
  }

  #[test]
  fn get_set_material() {
    let mut plane = Plane::default();
    let material = Material::new(Color::cyan(), 0.1, 0.4, 0.5, 50.);
    plane.set_material(material);
    assert_eq!(plane.material(), &material);
  }

  #[test]
  fn get_set_transformation() {
    let mut plane = Plane::default();
    let translation = Matrix::identity().translate(5., 4., 3.);
    plane.set_transformation(translation);
    assert_eq!(plane.transformation(), &translation);
  }

  #[test]
  fn normal() {
    let plane = Plane::default();
    assert_eq!(plane.normal_at(&Point::origin()), Vector::new(0., 1., 0.));
    assert_eq!(
      plane.normal_at(&Point::new(10., 0., -10.)),
      Vector::new(0., 1., 0.)
    );
    assert_eq!(
      plane.normal_at(&Point::new(-5., 0., 150.)),
      Vector::new(0., 1., 0.)
    );
  }

  #[test]
  fn normal_translated_scaled_plane() {
    let plane = Plane::new(
      Material::default(),
      Matrix::identity().translate(5., -5., 3.).scale(3., 0.3, 3.),
    );
    assert_eq!(plane.normal_at(&Point::origin()), Vector::new(0., 1., 0.));
  }

  #[test]
  fn normal_rotated_plane() {
    let plane = Plane::new(Material::default(), Matrix::identity().rotate_x(PI / 2.));
    assert_eq!(plane.normal_at(&Point::origin()), Vector::new(0., 0., 1.));

    let plane = Plane::new(Material::default(), Matrix::identity().rotate_x(-PI / 2.));
    assert_eq!(plane.normal_at(&Point::origin()), Vector::new(0., 0., -1.));
  }

  #[test]
  fn normal_is_normalized() {
    let plane = Plane::default();
    let normal = plane.normal_at(&Point::origin());
    assert_eq!(normal.normalize(), normal);
  }
}
