use core::shape::Shape;
use core::light::Light;

pub struct Scene{
    pub shapes: Vec<Box<Shape>>,
    pub lights: Vec<Box<Light>>,
}
impl Scene{
    pub fn new(shapes:Vec<Box<Shape>>, lights:Vec<Box<Light>>)-> Scene{
        Scene{shapes, lights}
    }
    pub fn intersect(self){
        for shape in self.shapes{
            shape.intersect();
        }
    }
}
