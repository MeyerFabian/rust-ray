use core::shape::Shape;
use cgmath::Vector3;
pub struct Sphere{
    pub origin:Vector3<f32>,
    pub radius:f32,
}
impl Sphere {
    pub fn new(origin:Vector3<f32> , radius:f32)-> Sphere{
        Sphere{origin,radius}
    }
}
impl Shape for Sphere{
    fn intersect(&self){
        println!("Sphere intersect.")
    }
}
