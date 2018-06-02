use core::material::Material;
pub struct Plastic {}
impl Plastic {
    pub fn new() -> Plastic {
        Plastic {}
    }
}
impl Material for Plastic {}
