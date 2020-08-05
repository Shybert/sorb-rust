use crate::canvas::Color;

struct Interaction {
  color: Color,
}
impl Interaction {
  fn new(color: Color) -> Self {
    return Self { color };
  }

  fn get_color(&self) -> &Color {
    return &self.color;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_color() {
    let color = Color::new(1., 0., 1.);
    let interaction = Interaction::new(color);
    assert_eq!(interaction.get_color(), &color);
  }
}
