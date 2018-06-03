use cgmath::Vector3;
pub trait Light: Sync {
    fn power(&self);
    fn position(&self) -> Vector3<f32>;
}
