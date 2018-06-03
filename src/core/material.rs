use cgmath::Vector3;
pub trait Material: Sync {
    fn color(&self) -> Vector3<u8>;
}
