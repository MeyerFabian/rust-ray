use core::math::Ray;
pub trait Shape {
    fn intersect(&self, &Ray) -> Option<f32>;
}
