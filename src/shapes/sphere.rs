use cgmath::{dot, InnerSpace, Vector3};
use core::math::*;
use core::shape::Shape;

pub struct Sphere {
    pub origin: Vector3<f32>,
    pub radius: f32,
}
impl Sphere {
    pub fn new(origin: Vector3<f32>, radius: f32) -> Sphere {
        Sphere { origin, radius }
    }
}
impl Shape for Sphere {
    fn intersect(&self, ray_world: &Ray) -> Option<f32> {
        let ray_local = Ray::new(ray_world.origin - self.origin, ray_world.direction);

        let radius_sqr = self.radius * self.radius;
        let order_0 = dot(ray_local.origin, ray_local.origin) - radius_sqr;
        let order_1 = 2.0 * dot(ray_local.origin, ray_local.direction);
        let order_2 = 1.0;
        let res = solve_quadr_eq(order_2, order_1, order_0);
        match res {
            Some((t1, t2)) => if t1 >= 0.0 {
                Some(t1)
            } else if t2 >= 0.0 {
                Some(t2)
            } else {
                None
            },
            None => None,
        }
    }

    fn intersectP(&self, ray_world: &Ray) -> Option<RayHit> {
        let ray_local = Ray::new(ray_world.origin - self.origin, ray_world.direction);

        let radius_sqr = self.radius * self.radius;
        let order_0 = dot(ray_local.origin, ray_local.origin) - radius_sqr;
        let order_1 = 2.0 * dot(ray_local.origin, ray_local.direction);
        let order_2 = 1.0;
        let res = solve_quadr_eq(order_2, order_1, order_0);
        match res {
            Some((t1, t2)) => if t1 >= 0.0 {
                let hit_pos = ray_world.origin + t1 * ray_world.direction;
                Some(RayHit::new(
                    t1,
                    hit_pos,
                    (hit_pos - self.origin).normalize(),
                ))
            } else if t2 >= 0.0 {
                let hit_pos = ray_world.origin + t1 * ray_world.direction;
                Some(RayHit::new(
                    t2,
                    hit_pos,
                    (hit_pos - self.origin).normalize(),
                ))
            } else {
                None
            },
            None => None,
        }
    }
}
