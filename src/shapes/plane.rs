use super::Shape;
use crate::geometry::{Material, Matrix, Point, Ray, Vector};

#[derive(Default)]
pub struct Plane {
    material: Material,
    transformation: Matrix,
}
impl Plane {
    pub fn new(material: Material, transformation: Matrix) -> Self {
        return Self {
            material,
            transformation,
        };
    }
}
impl Shape for Plane {
    fn material(&self) -> &Material {
        return &self.material;
    }
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn transformation(&self) -> &Matrix {
        return &self.transformation;
    }
    fn set_transformation(&mut self, transformation: Matrix) {
        self.transformation = transformation;
    }

    fn intersect_object_space(&self, ray: &Ray) -> Vec<f64> {
        todo!();
    }

    fn normal_at_object_space(&self, point: &Point) -> Vector {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn init_new() {
        let material = Material::new(Color::yellow(), 0.3, 0.3, 0.3, 70.);
        let scaling = Matrix::identity().scale(2., 2., 2.);
        let plane = Plane::new(material, scaling);
        assert_eq!(plane.material(), &material);
        assert_eq!(plane.transformation(), &scaling);
    }

    #[test]
    fn init_default() {
        let plane = Plane::default();
        assert_eq!(plane.material(), &Material::default());
        assert_eq!(plane.transformation(), &Matrix::identity());
    }

    #[test]
    fn get_set_material() {
        let mut plane = Plane::default();
        let material = Material::new(Color::cyan(), 0.1, 0.4, 0.5, 50.);
        plane.set_material(material);
        assert_eq!(plane.material(), &material);
    }

    #[test]
    fn get_set_transformation() {
        let mut plane = Plane::default();
        let translation = Matrix::identity().translate(5., 4., 3.);
        plane.set_transformation(translation);
        assert_eq!(plane.transformation(), &translation);
    }
}
