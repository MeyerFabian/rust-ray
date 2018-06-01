use core::shape::Shape;
use core::light::Light;
use core::material::Material;

pub struct Scene{
    pub shapes: Vec<Box<Shape>>,
    pub materials: Vec<Box<Material>>,
    pub lights: Vec<Box<Light>>,
}
impl Scene{
    pub fn new(shapes:Vec<Box<Shape>>, materials:Vec<Box<Material>>, lights:Vec<Box<Light>>)-> Scene{
        Scene{shapes,materials, lights}
    }
    pub fn intersect(&self){
        for shape in self.shapes.iter(){
            shape.intersect();
        }
    }
}
