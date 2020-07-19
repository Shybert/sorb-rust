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
}
