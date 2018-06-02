use core::material::Material;
use core::shape::Shape;
pub struct Primitive {
    pub shape: Box<Shape>,
    pub material: Box<Material>,
}
impl Primitive {
    pub fn new(shape: Box<Shape>, material: Box<Material>) -> Primitive {
        Primitive { shape, material }
    }
}
