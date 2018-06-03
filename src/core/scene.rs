use core::light::Light;
use core::math::Ray;
use core::math::RayHit;
use core::primitive::Primitive;
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
                Some(cand) => match acc {
                    Some(cur_min) => if cand < cur_min {
                        t
                    } else {
                        acc
                    },
                    None => t,
                },
                None => acc,
            })
    }
    pub fn intersectP(&self, ray: &Ray) -> Option<(usize, RayHit)> {
        self.primitives
            .iter()
            .enumerate()
            .map(|(i, primitive)| primitive.shape.intersectP(&ray).map(|ray_hit| (i, ray_hit)))
            .fold(None, |acc, t| match t {
                Some(cand) => match acc {
                    Some(cur_min) => if cand.1.distance < cur_min.1.distance {
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
