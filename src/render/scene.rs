use crate::render::PointLight;
use crate::shapes::Shape;

#[derive(Default)]
pub struct Scene {
  objects: Vec<Box<dyn Shape>>,
  lights: Vec<PointLight>,
}
impl Scene {
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
    let scene = Scene::new(
      vec![Box::new(Sphere::default()), Box::new(Sphere::default())],
      vec![PointLight::default(); 3],
    );
    assert_eq!(scene.objects().len(), 2);
    assert_eq!(scene.lights().len(), 3);
  }

  #[test]
  fn init_default() {
    let scene = Scene::default();
    assert_eq!(scene.objects().len(), 0);
    assert_eq!(scene.lights().len(), 0);
  }
}
