use cgmath::Vector3;
use core::material::Material;
pub struct Plastic {
    pub color: Vector3<u8>,
}
impl Plastic {
    pub fn new(color: Vector3<u8>) -> Plastic {
        Plastic { color }
    }
}
impl Material for Plastic {
    fn color(&self) -> Vector3<u8> {
        self.color
    }
}
