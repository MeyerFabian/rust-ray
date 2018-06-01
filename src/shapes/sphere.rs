use core::shape::Shape;
pub struct Sphere{
    pub radius:f32,
}
impl Sphere {
    pub fn new(radius:f32)-> Sphere{
        Sphere{radius}
    }
}
impl Shape for Sphere{
    fn intersect(&self){
        println!("Sphere intersect.")
    }
}
