use core::math::{Ray, RayHit};
pub trait Shape: Sync {
    fn intersect(&self, &Ray) -> Option<f32>;
    fn intersectP(&self, &Ray) -> Option<RayHit>;
}
