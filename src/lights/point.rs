use core::light::Light;
use cgmath::{Vector3};
pub struct PointLight{
    pub origin: Vector3<f32>,
}
impl PointLight{
    pub fn new(origin:Vector3<f32>) -> PointLight{
        PointLight{origin}
    }
    
}
impl Light for PointLight{
    fn power(&self){
        //TODO give back power
    }
}
