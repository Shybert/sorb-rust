use crate::geometry::{Material, Point, Vector};
use crate::render::PointLight;
use crate::Color;

pub fn lighting(
  material: &Material,
  position: &Point,
  light: &PointLight,
  eye_vector: &Vector,
  normal: &Vector,
  in_shadow: bool,
) -> Color {
  let effective_color = material.color_at(position) * *light.color();
  let light_to_point = (*light.position() - *position).normalize();

  let ambient = effective_color * material.ambience();
  let mut diffuse = Color::black();
  let mut specular = Color::black();

  let light_dot_normal = light_to_point.dot(&normal);
  if !in_shadow && light_dot_normal > 0. {
    diffuse = light_dot_normal * material.diffuse() * effective_color;

    let reflection = (-light_to_point).reflect(&normal);
    let reflection_dot_eye = eye_vector.dot(&reflection);
    if reflection_dot_eye > 0. {
      specular =
        reflection_dot_eye.powf(material.shininess()) * material.specular() * *light.color();
    };
  };

  return ambient + diffuse + specular;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn eye_between_light_and_surface() {
    let eye_vector = Vector::new(0., 0., -1.);
    let normal = Vector::new(0., 0., -1.);
    let light = PointLight::new(Point::new(0., 0., -10.), Color::white());

    let result = lighting(
      &Material::default(),
      &Point::origin(),
      &light,
      &eye_vector,
      &normal,
      false,
    );
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
  }

  #[test]
  fn eye_light_eye_offset_45_degree() {
    let eye_vector = Vector::new(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.);
    let normal = Vector::new(0., 0., -1.);
    let light = PointLight::new(Point::new(0., 0., -10.), Color::white());

    let result = lighting(
      &Material::default(),
      &Point::origin(),
      &light,
      &eye_vector,
      &normal,
      false,
    );
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
  }

  #[test]
  fn eye_light_light_offset_45_degree() {
    let eye_vector = Vector::new(0., 0., -1.);
    let normal = Vector::new(0., 0., -1.);
    let light = PointLight::new(Point::new(0., 10., -10.), Color::white());

    let result = lighting(
      &Material::default(),
      &Point::origin(),
      &light,
      &eye_vector,
      &normal,
      false,
    );
    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
  }

  #[test]
  fn eye_light_both_offset() {
    let eye_vector = Vector::new(0., -10., -10.).normalize();
    let normal = Vector::new(0., 0., -1.);
    let light = PointLight::new(Point::new(0., 10., -10.), Color::white());

    let result = lighting(
      &Material::default(),
      &Point::origin(),
      &light,
      &eye_vector,
      &normal,
      false,
    );
    assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
  }

  #[test]
  fn light_behind_surface() {
    let eye_vector = Vector::new(0., 0., -1.);
    let normal = Vector::new(0., 0., -1.);
    let light = PointLight::new(Point::new(0., 0., 10.), Color::white());

    let result = lighting(
      &Material::default(),
      &Point::origin(),
      &light,
      &eye_vector,
      &normal,
      false,
    );
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
  }

  #[test]
  fn surface_in_shadow() {
    let eye_vector = Vector::new(0., 0., -1.);
    let normal = Vector::new(0., 0., -1.);
    let light = PointLight::new(Point::new(0., 0., -10.), Color::white());

    let result = lighting(
      &Material::default(),
      &Point::origin(),
      &light,
      &eye_vector,
      &normal,
      true,
    );
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
  }
}
