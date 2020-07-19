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
}
