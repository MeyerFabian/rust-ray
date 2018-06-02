use core::light::Light;
use core::math::Ray;
use core::primitive::Primitive;
use std::f32::INFINITY;
pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Box<Light>>,
}
impl Scene {
    pub fn new(primitives: Vec<Primitive>, lights: Vec<Box<Light>>) -> Scene {
        Scene { primitives, lights }
    }
    pub fn intersect(&self, ray: &Ray) -> f32 {
        let mut closest_intersect: f32 = 1.0;
        for primitive in self.primitives.iter() {
            let hit = primitive.shape.intersect(&ray);
            match hit {
                Some(x) => closest_intersect = closest_intersect.min(x),
                None => (),
            }
        }
        closest_intersect
    }
}
