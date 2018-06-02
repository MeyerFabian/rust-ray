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
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        self.primitives
            .iter()
            .map(|primitive| primitive.shape.intersect(&ray))
            .fold(None, |acc, t| match t {
                Some(cand) => match (acc) {
                    Some(cur_min) => if cand < cur_min {
                        Some(cand)
                    } else {
                        Some(cur_min)
                    },
                    None => Some(cand),
                },
                None => acc,
            })
    }
}
