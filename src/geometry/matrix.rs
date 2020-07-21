use crate::geometry::vector::Vector;
use crate::utils::approx_equals;
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Copy, Debug)]
struct Matrix {
  elements: [[f64; 4]; 4],
}
impl Matrix {
  fn new() -> Self {
    return Matrix {
      elements: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
      ],
    };
  }

  fn from(elements: [[f64; 4]; 4]) -> Self {
    return Self { elements };
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
impl PartialEq<Self> for Matrix {
  fn eq(&self, other: &Self) -> bool {
    for row in 0..4 {
      for col in 0..4 {
        if !(approx_equals(&self[(row, col)], &other[(row, col)])) {
          return false;
        }
      }
    }
    return true;
  }
}
impl Mul<Self> for Matrix {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    let mut new_matrix = Matrix::new();
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn init_new() {
    assert_eq!(
      Matrix::new().elements,
      [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
      ]
    );
  }

  #[test]
  fn init_from() {
    let elements = [
      [1., 2., 3., 4.],
      [5.5, 6.5, 7.5, 8.5],
      [9., 10., 11., 12.],
      [13.5, 14.5, 15.5, 16.5],
    ];

    assert_eq!(Matrix::from(elements).elements, elements);
  }

  #[test]
  fn index_get() {
    let matrix = Matrix::new();
    assert_eq!(matrix[(0, 0)], 1.);
    assert_eq!(matrix[(2, 1)], 0.);
  }

  #[test]
  fn index_set() {
    let mut matrix = Matrix::new();
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
    let a = Matrix::from(elements);
    let mut b = Matrix::from(elements);

    assert_eq!(a, b);
    b[(0, 0)] = 1.000000001;
    assert_eq!(a, b);
  }

  #[test]
  fn equality_different() {
    let a = Matrix::from([
      [1., 2., 3., 4.],
      [5., 6., 7., 8.],
      [9., 8., 7., 6.],
      [5., 4., 3., 2.],
    ]);
    let b = Matrix::from([
      [2., 3., 4., 5.],
      [6., 7., 8., 9.],
      [8., 7., 6., 5.],
      [4., 3., 2., 1.],
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn matrix_matrix_multiplication() {
    let a = Matrix::from([
      [1., 2., 3., 4.],
      [5., 6., 7., 8.],
      [9., 8., 7., 6.],
      [5., 4., 3., 2.],
    ]);
    let b = Matrix::from([
      [-2., 1., 2., 3.],
      [3., 2., 1., -1.],
      [4., 3., 6., 5.],
      [1., 2., 7., 8.],
    ]);
    let c = Matrix::from([
      [2., 4., 6., 8.],
      [1., 2., 3., 4.],
      [4., 3., 2., 1.],
      [8., 6., 4., 2.],
    ]);

    let expected = Matrix::from([
      [20., 22., 50., 48.],
      [44., 54., 114., 108.],
      [40., 58., 110., 102.],
      [16., 26., 46., 42.],
    ]);
    assert_eq!(a * b, expected);

    let expected = Matrix::from([
      [646., 562., 478., 394.],
      [1462., 1274., 1086., 898.],
      [1394., 1218., 1042., 866.],
      [578., 506., 434., 362.],
    ]);
    assert_eq!(a * b * c, expected);
  }

  #[test]
  fn matrix_vector_multiplication() {
    let matrix = Matrix::from([
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
  fn identity_matrix_does_nothing() {
    let identity = Matrix::new();
    let matrix = Matrix::from([
      [0., 1., 2., 4.],
      [1., 2., 4., 8.],
      [2., 4., 8., 16.],
      [4., 8., 16., 32.],
    ]);
    let vector = Vector::from(1., 2., 3.);

    assert_eq!(identity * matrix, matrix);
    assert_eq!(matrix * identity, matrix);
    assert_eq!(identity * vector, vector);
  }
}
