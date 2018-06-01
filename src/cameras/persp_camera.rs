use cgmath::{Vector3};
pub struct PerspectiveCamera{
    pub origin: Vector3<f32>,
}
impl PerspectiveCamera{
    pub fn new(origin: Vector3<f32>) -> PerspectiveCamera{
        PerspectiveCamera{origin}
    }
}
