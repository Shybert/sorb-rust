use crate::shapes::Shape;

#[derive(Default)]
pub struct Scene {
  objects: Vec<Box<dyn Shape>>,
}
impl Scene {
  pub fn new(objects: Vec<Box<dyn Shape>>) -> Self {
    return Self { objects };
  }

  pub fn get_objects(&self) -> &[Box<dyn Shape>] {
    return &self.objects;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::shapes::Sphere;

  #[test]
  fn init_new() {
    let scene = Scene::new(vec![
      Box::new(Sphere::default()),
      Box::new(Sphere::default()),
    ]);
    assert_eq!(scene.get_objects().len(), 2);
  }

  #[test]
  fn init_default() {
    let scene = Scene::default();
    assert_eq!(scene.get_objects().len(), 0);
  }
}
