use cgmath::Vector2;
use core::math::Ray;
pub trait Camera {
    fn generate_ray(&self, pixel: Vector2<u32>) -> Ray;
    fn get_viewport(&self) -> Vector2<u32>;
    fn write_image(&self, &[u8]);
}
