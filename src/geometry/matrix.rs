use crate::geometry::{Point, Vector};
use crate::utils::approx_equals;
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
  elements: [[f64; 4]; 4],
}
impl Matrix {
  pub fn identity() -> Self {
    return Self {
      elements: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
      ],
    };
  }

  pub fn new(elements: [[f64; 4]; 4]) -> Self {
    return Self { elements };
  }

  pub fn transpose(&self) -> Self {
    return Self::new([
      [self[(0, 0)], self[(1, 0)], self[(2, 0)], self[(3, 0)]],
      [self[(0, 1)], self[(1, 1)], self[(2, 1)], self[(3, 1)]],
      [self[(0, 2)], self[(1, 2)], self[(2, 2)], self[(3, 2)]],
      [self[(0, 3)], self[(1, 3)], self[(2, 3)], self[(3, 3)]],
    ]);
  }

  pub fn inverse(&self) -> Self {
    let mut inverse = Self::identity();
    let mut copy = self.clone();

    for i in 0..4 {
      let mut pivot = copy[(i, i)];

      if pivot == 0. {
        for j in i..4 {
          if copy[(j, i)] != 0. {
            copy.elements.swap(i, j);
            inverse.elements.swap(i, j);
          }
        }

        pivot = copy[(i, i)];
        if pivot == 0. {
          panic!("Matrix is singular and not invertible");
        }
      }

      if pivot != 1. {
        for j in 0..4 {
          copy[(i, j)] /= pivot;
          inverse[(i, j)] /= pivot;
        }
      }

      for j in 0..4 {
        if j != i && copy[(j, i)] != 0. {
          let scalar = copy[(j, i)];
          for k in 0..4 {
            copy[(j, k)] -= scalar * copy[(i, k)];
            inverse[(j, k)] -= scalar * inverse[(i, k)];
          }
        }
      }
    }

    return inverse;
  }

  pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
    let translation = Self::new([
      [1., 0., 0., x],
      [0., 1., 0., y],
      [0., 0., 1., z],
      [0., 0., 0., 1.],
    ]);

    return translation * self;
  }

  pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
    let scaling = Self::new([
      [x, 0., 0., 0.],
      [0., y, 0., 0.],
      [0., 0., z, 0.],
      [0., 0., 0., 1.],
    ]);

    return scaling * self;
  }

  pub fn rotate_x(self, angle: f64) -> Self {
    let (angle_sin, angle_cos) = angle.sin_cos();
    let rotation = Self::new([
      [1., 0., 0., 0.],
      [0., angle_cos, -angle_sin, 0.],
      [0., angle_sin, angle_cos, 0.],
      [0., 0., 0., 1.],
    ]);

    return rotation * self;
  }

  pub fn rotate_y(self, angle: f64) -> Self {
    let (angle_sin, angle_cos) = angle.sin_cos();
    let rotation = Self::new([
      [angle_cos, 0., angle_sin, 0.],
      [0., 1., 0., 0.],
      [-angle_sin, 0., angle_cos, 0.],
      [0., 0., 0., 1.],
    ]);

    return rotation * self;
  }

  pub fn rotate_z(self, angle: f64) -> Self {
    let (angle_sin, angle_cos) = angle.sin_cos();
    let rotation = Self::new([
      [angle_cos, -angle_sin, 0., 0.],
      [angle_sin, angle_cos, 0., 0.],
      [0., 0., 1., 0.],
      [0., 0., 0., 1.],
    ]);

    return rotation * self;
  }

  pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
    let shear = Self::new([
      [1., xy, xz, 0.],
      [yx, 1., yz, 0.],
      [zx, zy, 1., 0.],
      [0., 0., 0., 1.],
    ]);

    return shear * self;
  }
}
impl Default for Matrix {
  fn default() -> Self {
    return Self::identity();
  }
}
impl Index<(usize, usize)> for Matrix {
  type Output = f64;

  fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
    return &self.elements[row][col];
  }
}
impl IndexMut<(usize, usize)> for Matrix {
  fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
    return &mut self.elements[row][col];
  }
}
impl PartialEq for Matrix {
  fn eq(&self, other: &Self) -> bool {
    for row in 0..4 {
      for col in 0..4 {
        if !(approx_equals(self[(row, col)], other[(row, col)])) {
          return false;
        }
      }
    }
    return true;
  }
}
impl Mul for Matrix {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    let mut new_matrix = Self::identity();
    for row in 0..4 {
      for col in 0..4 {
        new_matrix[(row, col)] = self[(row, 0)] * rhs[(0, col)]
          + self[(row, 1)] * rhs[(1, col)]
          + self[(row, 2)] * rhs[(2, col)]
          + self[(row, 3)] * rhs[(3, col)]
      }
    }
    return new_matrix;
  }
}
impl Mul<Vector> for Matrix {
  type Output = Vector;

  fn mul(self, vector: Vector) -> Vector {
    return Vector::from(
      vector.x * self[(0, 0)] + vector.y * self[(0, 1)] + vector.z * self[(0, 2)],
      vector.x * self[(1, 0)] + vector.y * self[(1, 1)] + vector.z * self[(1, 2)],
      vector.x * self[(2, 0)] + vector.y * self[(2, 1)] + vector.z * self[(2, 2)],
    );
  }
}
impl Mul<Point> for Matrix {
  type Output = Point;

  fn mul(self, point: Point) -> Point {
    return Point::from(
      point.x * self[(0, 0)] + point.y * self[(0, 1)] + point.z * self[(0, 2)] + self[(0, 3)],
      point.x * self[(1, 0)] + point.y * self[(1, 1)] + point.z * self[(1, 2)] + self[(1, 3)],
      point.x * self[(2, 0)] + point.y * self[(2, 1)] + point.z * self[(2, 2)] + self[(2, 3)],
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f64::consts::PI;

  #[test]
  fn init_identity() {
    assert_eq!(
      Matrix::identity().elements,
      [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
      ]
    );
  }

  #[test]
  fn init_default() {
    assert_eq!(Matrix::default(), Matrix::identity());
  }

  #[test]
  fn init_new() {
    let elements = [
      [1., 2., 3., 4.],
      [5.5, 6.5, 7.5, 8.5],
      [9., 10., 11., 12.],
      [13.5, 14.5, 15.5, 16.5],
    ];

    assert_eq!(Matrix::new(elements).elements, elements);
  }

  #[test]
  fn index_get() {
    let matrix = Matrix::identity();
    assert_eq!(matrix[(0, 0)], 1.);
    assert_eq!(matrix[(2, 1)], 0.);
  }

  #[test]
  fn index_set() {
    let mut matrix = Matrix::identity();
    matrix[(1, 2)] = 1.5;
    assert_eq!(matrix[(1, 2)], 1.5);
  }

  #[test]
  fn equality_identical() {
    let elements = [
      [1., 2., 3., 4.],
      [5., 6., 7., 8.],
      [9., 8., 7., 6.],
      [5., 4., 3., 2.],
    ];
    let a = Matrix::new(elements);
    let mut b = Matrix::new(elements);

    assert_eq!(a, b);
    b[(0, 0)] = 1.000000001;
    assert_eq!(a, b);
  }

  #[test]
  fn equality_different() {
    let a = Matrix::new([
      [1., 2., 3., 4.],
      [5., 6., 7., 8.],
      [9., 8., 7., 6.],
      [5., 4., 3., 2.],
    ]);
    let b = Matrix::new([
      [2., 3., 4., 5.],
      [6., 7., 8., 9.],
      [8., 7., 6., 5.],
      [4., 3., 2., 1.],
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn matrix_matrix_multiplication() {
    let a = Matrix::new([
      [1., 2., 3., 4.],
      [5., 6., 7., 8.],
      [9., 8., 7., 6.],
      [5., 4., 3., 2.],
    ]);
    let b = Matrix::new([
      [-2., 1., 2., 3.],
      [3., 2., 1., -1.],
      [4., 3., 6., 5.],
      [1., 2., 7., 8.],
    ]);
    let c = Matrix::new([
      [2., 4., 6., 8.],
      [1., 2., 3., 4.],
      [4., 3., 2., 1.],
      [8., 6., 4., 2.],
    ]);

    let expected = Matrix::new([
      [20., 22., 50., 48.],
      [44., 54., 114., 108.],
      [40., 58., 110., 102.],
      [16., 26., 46., 42.],
    ]);
    assert_eq!(a * b, expected);

    let expected = Matrix::new([
      [646., 562., 478., 394.],
      [1462., 1274., 1086., 898.],
      [1394., 1218., 1042., 866.],
      [578., 506., 434., 362.],
    ]);
    assert_eq!(a * b * c, expected);
  }

  #[test]
  fn matrix_vector_multiplication() {
    let matrix = Matrix::new([
      [1., 2., 3., 4.],
      [2., 4., 4., 2.],
      [8., 6., 4., 1.],
      [0., 0., 0., 1.],
    ]);
    let vector = Vector::from(1., 2., 3.);

    let expected = Vector::from(14., 22., 32.);
    assert_eq!(matrix * vector, expected);
  }

  #[test]
  fn matrix_point_multiplication() {
    let matrix = Matrix::new([
      [1., 2., 3., 4.],
      [2., 4., 4., 2.],
      [8., 6., 4., 2.],
      [0., 0., 0., 1.],
    ]);
    let point = Point::from(1., 2., 3.);

    let expected = Point::from(18., 24., 34.);
    assert_eq!(matrix * point, expected);
  }

  #[test]
  fn transpose() {
    let matrix = Matrix::new([
      [0., 9., 3., 0.],
      [9., 8., 0., 8.],
      [1., 8., 5., 3.],
      [0., 0., 5., 8.],
    ]);
    let transposed_matrix = Matrix::new([
      [0., 9., 1., 0.],
      [9., 8., 8., 0.],
      [3., 0., 5., 5.],
      [0., 8., 3., 8.],
    ]);

    assert_eq!(matrix.transpose(), transposed_matrix);
    assert_eq!(matrix.transpose().transpose(), matrix);
  }

  #[test]
  fn invert() {
    let matrix = Matrix::new([
      [-5., 2., 6., -8.],
      [1., -5., 1., 8.],
      [7., 7., -6., -7.],
      [1., -3., 7., 4.],
    ]);
    let inverse = Matrix::new([
      [0.21805, 0.45113, 0.24060, -0.04511],
      [-0.80827, -1.45677, -0.44361, 0.52068],
      [-0.07895, -0.22368, -0.05263, 0.19737],
      [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);
    assert_eq!(matrix.inverse(), inverse);

    let matrix = Matrix::new([
      [8., -5., 9., 2.],
      [7., 5., 6., 1.],
      [-6., 0., 9., 6.],
      [-3., 0., -9., -4.],
    ]);
    let inverse = Matrix::new([
      [-0.15385, -0.15385, -0.28205, -0.53846],
      [-0.07692, 0.12308, 0.02564, 0.03077],
      [0.35897, 0.35897, 0.43590, 0.92308],
      [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);
    assert_eq!(matrix.inverse(), inverse);

    let matrix = Matrix::new([
      [9., 3., 0., 9.],
      [-5., -2., -6., -3.],
      [-4., 9., 6., 4.],
      [-7., 6., 6., 2.],
    ]);
    let inverse = Matrix::new([
      [-0.04074, -0.07778, 0.14444, -0.22222],
      [-0.07778, 0.03333, 0.36667, -0.33333],
      [-0.02901, -0.14630, -0.10926, 0.12963],
      [0.17778, 0.06667, -0.26667, 0.33333],
    ]);
    assert_eq!(matrix.inverse(), inverse);
  }

  #[test]
  #[should_panic(expected = "singular")]
  fn invert_panics_if_singular_matrix() {
    let matrix = Matrix::new([
      [-4., 2., -2., -3.],
      [9., 6., 2., 6.],
      [0., -5., 1., -5.],
      [0., 0., 0., 0.],
    ]);
    matrix.inverse();
  }

  #[test]
  fn multiplying_by_inverse_gives_identity_matrix() {
    let identity = Matrix::identity();
    let a = Matrix::new([
      [3., -9., 7., 3.],
      [3., -8., 2., -9.],
      [-4., 4., 4., 1.],
      [-6., 5., -1., 1.],
    ]);
    let b = Matrix::new([
      [8., 2., 2., 2.],
      [3., -1., 7., 0.],
      [7., 0., 5., 4.],
      [6., -2., 0., 5.],
    ]);

    assert_eq!(a * a.inverse(), identity);
    assert_eq!(a.inverse() * a, identity);
    assert_eq!(a * b * b.inverse(), a);
  }

  #[test]
  fn multiplying_identity_matrix_does_nothing() {
    let identity = Matrix::identity();
    let matrix = Matrix::new([
      [0., 1., 2., 4.],
      [1., 2., 4., 8.],
      [2., 4., 8., 16.],
      [4., 8., 16., 32.],
    ]);
    let vector = Vector::from(1., 2., 3.);

    assert_eq!(identity * identity, identity);
    assert_eq!(identity * matrix, matrix);
    assert_eq!(matrix * identity, matrix);
    assert_eq!(identity * vector, vector);
  }

  #[test]
  fn transposing_identity_matrix_does_nothing() {
    let identity = Matrix::identity();
    assert_eq!(identity.transpose(), identity);
  }
  #[test]
  fn inverting_identity_matrix_does_nothing() {
    let identity = Matrix::identity();
    assert_eq!(identity.inverse(), identity);
  }

  #[test]
  fn translation_moves_points() {
    let translation = Matrix::identity().translate(5., -3., 2.);
    let point = Point::from(-3., 4., 5.);
    assert_eq!(translation * point, Point::from(2., 1., 7.));
  }

  #[test]
  fn translation_inverse_moves_points_in_reverse() {
    let translation = Matrix::identity().translate(5., -3., 2.);
    let point = Point::from(-3., 4., 5.);
    assert_eq!(translation.inverse() * point, Point::from(-8., 7., 3.));
  }

  #[test]
  fn translation_does_not_affect_vectors() {
    let translation = Matrix::identity().translate(5., -3., 2.);
    let vector = Vector::from(-3., 4., 5.);
    assert_eq!(translation * vector, vector);
  }

  #[test]
  fn scaling_scales_points() {
    let scaling = Matrix::identity().scale(2., 3., 4.);
    let point = Point::from(-4., 6., 8.);
    assert_eq!(scaling * point, Point::from(-8., 18., 32.));
  }

  #[test]
  fn scaling_scales_vectors() {
    let scaling = Matrix::identity().scale(2., 3., 4.);
    let vector = Vector::from(-4., 6., 8.);
    assert_eq!(scaling * vector, Vector::from(-8., 18., 32.));
  }

  #[test]
  fn scaling_inverse_scales_in_reverse() {
    let scaling = Matrix::identity().scale(2., 3., 4.);
    let vector = Vector::from(-4., 6., 8.);
    assert_eq!(scaling.inverse() * vector, Vector::from(-2., 2., 2.));
  }

  #[test]
  fn scaling_by_negative_value_is_reflection() {
    let reflection = Matrix::identity().scale(-1., 1., 1.);
    let vector = Vector::from(2., 3., 4.);
    assert_eq!(reflection * vector, Vector::from(-2., 3., 4.));
  }

  #[test]
  fn rotate_x() {
    let point = Point::from(0., 1., 0.);
    let quarter_rotation = Matrix::identity().rotate_x(PI / 4.);
    let half_rotation = Matrix::identity().rotate_x(PI / 2.);

    assert_eq!(
      quarter_rotation * point,
      Point::from(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
    );
    assert_eq!(half_rotation * point, Point::from(0., 0., 1.),);
  }

  #[test]
  fn rotate_x_inverse_rotates_in_reverse() {
    let point = Point::from(0., 1., 0.);
    let half_rotation = Matrix::identity().rotate_x(PI / 2.);
    assert_eq!(half_rotation.inverse() * point, Point::from(0., 0., -1.,));
  }

  #[test]
  fn rotate_y() {
    let point = Point::from(0., 0., 1.);
    let quarter_rotation = Matrix::identity().rotate_y(PI / 4.);
    let half_rotation = Matrix::identity().rotate_y(PI / 2.);

    assert_eq!(
      quarter_rotation * point,
      Point::from(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.),
    );
    assert_eq!(half_rotation * point, Point::from(1., 0., 0.),);
  }

  #[test]
  fn rotate_y_inverse_rotates_in_reverse() {
    let point = Point::from(0., 0., 1.);
    let half_rotation = Matrix::identity().rotate_y(PI / 2.);
    assert_eq!(half_rotation.inverse() * point, Point::from(-1., 0., 0.,));
  }

  #[test]
  fn rotate_z() {
    let point = Point::from(1., 0., 0.);
    let quarter_rotation = Matrix::identity().rotate_z(PI / 4.);
    let half_rotation = Matrix::identity().rotate_z(PI / 2.);

    assert_eq!(
      quarter_rotation * point,
      Point::from(2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.),
    );
    assert_eq!(half_rotation * point, Point::from(0., 1., 0.),);
  }

  #[test]
  fn rotate_z_inverse_rotates_in_reverse() {
    let point = Point::from(1., 0., 0.);
    let half_rotation = Matrix::identity().rotate_z(PI / 2.);
    assert_eq!(half_rotation.inverse() * point, Point::from(0., -1., 0.,));
  }

  #[test]
  fn shear() {
    let point = Point::from(2., 3., 4.);

    let shear = Matrix::identity().shear(1., 0., 0., 0., 0., 0.);
    assert_eq!(shear * point, Point::from(5., 3., 4.));

    let shear = Matrix::identity().shear(0., 1., 0., 0., 0., 0.);
    assert_eq!(shear * point, Point::from(6., 3., 4.));

    let shear = Matrix::identity().shear(0., 0., 1., 0., 0., 0.);
    assert_eq!(shear * point, Point::from(2., 5., 4.));

    let shear = Matrix::identity().shear(0., 0., 0., 1., 0., 0.);
    assert_eq!(shear * point, Point::from(2., 7., 4.));

    let shear = Matrix::identity().shear(0., 0., 0., 0., 1., 0.);
    assert_eq!(shear * point, Point::from(2., 3., 6.));

    let shear = Matrix::identity().shear(0., 0., 0., 0., 0., 1.);
    assert_eq!(shear * point, Point::from(2., 3., 7.));
  }

  #[test]
  fn chained_transformations() {
    let point = Point::from(1., 0., 1.);
    let transform = Matrix::identity()
      .rotate_x(PI / 2.)
      .scale(5., 5., 5.)
      .translate(10., 5., 7.);

    assert_eq!(transform * point, Point::from(15., 0., 7.));
  }
}
