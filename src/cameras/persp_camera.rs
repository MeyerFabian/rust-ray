use cgmath::{InnerSpace, Vector2, Vector3};
use core::camera::Camera;
use core::math::Ray;
use image;
pub struct PerspectiveCamera {
    pub origin: Vector3<f32>,
    pub lookAt: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub viewport: Vector2<u32>,
}
impl PerspectiveCamera {
    pub fn new(
        origin: Vector3<f32>,
        lookAt: Vector3<f32>,
        up: Vector3<f32>,
        right: Vector3<f32>,
        viewport: Vector2<u32>,
    ) -> PerspectiveCamera {
        PerspectiveCamera {
            origin,
            lookAt,
            up,
            right,
            viewport,
        }
    }
}
impl Camera for PerspectiveCamera {
    fn generate_ray(&self, pixel: Vector2<u32>) -> Ray {
        let aspect_ratio = self.viewport.x as f32 / self.viewport.y as f32;
        let xpart = ((pixel.x as f32) / (self.viewport.x as f32) - 0.5) * aspect_ratio;
        let ypart = (pixel.y as f32) / (self.viewport.y as f32) - 0.5;

        Ray::new(self.origin, Vector3::new(xpart, ypart, -1.0).normalize())
    }
    fn get_viewport(&self) -> Vector2<u32> {
        self.viewport
    }
    fn write_image(&self, buffer: &[u8]) {
        image::save_buffer(
            "image.png",
            &buffer,
            self.viewport.x,
            self.viewport.y,
            image::RGB(8),
        ).unwrap()
    }
}
