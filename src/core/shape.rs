use core::math::Ray;
pub trait Shape: Sync {
    fn intersect(&self, &Ray) -> Option<f32>;
}
