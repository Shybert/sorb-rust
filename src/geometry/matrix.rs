use crate::utils::approx_equals;

#[derive(Debug)]
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
impl PartialEq<Self> for Matrix {
  fn eq(&self, other: &Self) -> bool {
    for i in 0..4 {
      for j in 0..4 {
        if !(approx_equals(&self.elements[i][j], &other.elements[i][j])) {
          return false;
        }
      }
    }
    return true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
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
  fn from() {
    let elements = [
      [1., 2., 3., 4.],
      [5.5, 6.5, 7.5, 8.5],
      [9., 10., 11., 12.],
      [13.5, 14.5, 15.5, 16.5],
    ];

    assert_eq!(Matrix::from(elements).elements, elements);
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
    b.elements[0][0] = 1.000000001;
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
}
