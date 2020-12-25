use crate::render::PointLight;
use crate::shapes::Shape;

#[derive(Default)]
pub struct World {
  objects: Vec<Box<dyn Shape>>,
  lights: Vec<PointLight>,
}
impl World {
  pub fn new(objects: Vec<Box<dyn Shape>>, lights: Vec<PointLight>) -> Self {
    return Self { objects, lights };
  }

  pub fn objects(&self) -> &[Box<dyn Shape>] {
    return &self.objects;
  }

  pub fn lights(&self) -> &[PointLight] {
    return &self.lights;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::shapes::Sphere;

  #[test]
  fn init_new() {
    let world = World::new(
      vec![Box::new(Sphere::default()), Box::new(Sphere::default())],
      vec![PointLight::default(); 3],
    );
    assert_eq!(world.objects().len(), 2);
    assert_eq!(world.lights().len(), 3);
  }

  #[test]
  fn init_default() {
    let world = World::default();
    assert_eq!(world.objects().len(), 0);
    assert_eq!(world.lights().len(), 0);
  }
}
