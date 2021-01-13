use crate::geometry::Vector;
use crate::Color;

pub fn phong(
  base_color: Color,
  (ambience, diffuse, specular, shininess): (f64, f64, f64, f64),
  (light_vector, normal, eye_vector): (Vector, Vector, Vector),
  light_color: Color,
  in_shadow: bool,
) -> Color {
  let effective_color = base_color * light_color;

  let ambient_color = effective_color * ambience;
  let mut diffuse_color = Color::black();
  let mut specular_color = Color::black();

  let light_dot_normal = light_vector.dot(&normal);
  if !in_shadow && light_dot_normal > 0. {
    diffuse_color = light_dot_normal * diffuse * effective_color;

    let reflection = (-light_vector).reflect(&normal);
    let reflection_dot_eye = eye_vector.dot(&reflection);
    if reflection_dot_eye > 0. {
      specular_color = reflection_dot_eye.powf(shininess) * specular * light_color;
    };
  };

  return ambient_color + diffuse_color + specular_color;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::geometry::{Material, Point, Vector};

  fn phong_default_material_white_light(
    (light_vector, normal, eye_vector): (Vector, Vector, Vector),
    in_shadow: bool,
  ) -> Color {
    return phong(
      Material::default().color_at(&Point::origin()),
      Material::default().shading_properties(),
      (light_vector, normal, eye_vector),
      Color::white(),
      in_shadow,
    );
  }

  #[test]
  fn phong_eye_between_light_and_surface() {
    let light_vector = Vector::new(0., 0., -1.);
    let normal = Vector::new(0., 0., -1.);
    let eye_vector = Vector::new(0., 0., -1.);

    let actual = phong_default_material_white_light((light_vector, normal, eye_vector), false);
    let expected = Color::new(1.9, 1.9, 1.9);
    assert_eq!(actual, expected);
  }

  #[test]
  fn phong_eye_light_eye_offset_45_degree() {
    let light_vector = Vector::new(0., 0., -1.);
    let normal = Vector::new(0., 0., -1.);
    let eye_vector = Vector::new(0., 1., 1.).normalize();

    let actual = phong_default_material_white_light((light_vector, normal, eye_vector), false);
    let expected = Color::new(1.0, 1.0, 1.0);
    assert_eq!(actual, expected);
  }

  #[test]
  fn phong_eye_light_light_offset_45_degree() {
    let light_vector = Vector::new(0., 1., -1.).normalize();
    let normal = Vector::new(0., 0., -1.);
    let eye_vector = Vector::new(0., 0., -1.);

    let actual = phong_default_material_white_light((light_vector, normal, eye_vector), false);
    let expected = Color::new(0.7364, 0.7364, 0.7364);
    assert_eq!(actual, expected);
  }

  #[test]
  fn phong_eye_light_both_offset() {
    let light_vector = Vector::new(0., 1., -1.).normalize();
    let normal = Vector::new(0., 0., -1.);
    let eye_vector = Vector::new(0., -1., -1.).normalize();

    let actual = phong_default_material_white_light((light_vector, normal, eye_vector), false);
    let expected = Color::new(1.6364, 1.6364, 1.6364);
    assert_eq!(actual, expected);
  }

  #[test]
  fn phong_light_behind_surface() {
    let light_vector = Vector::new(0., 0., 1.);
    let normal = Vector::new(0., 0., -1.);
    let eye_vector = Vector::new(0., 0., -1.);

    let actual = phong_default_material_white_light((light_vector, normal, eye_vector), false);
    let expected = Color::new(0.1, 0.1, 0.1);
    assert_eq!(actual, expected);
  }

  #[test]
  fn phong_surface_in_shadow() {
    let light_vector = Vector::new(0., 0., 1.);
    let normal = Vector::new(0., 0., -1.);
    let eye_vector = Vector::new(0., 0., -1.);
    let in_shadow = true;

    let actual = phong_default_material_white_light((light_vector, normal, eye_vector), in_shadow);
    let expected = Color::new(0.1, 0.1, 0.1);
    assert_eq!(actual, expected);
  }
}
