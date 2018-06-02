use cgmath::{Vector2, Vector3};
use core::camera::Camera;
use core::integrator::Integrator;
use core::math::Ray;
use core::scene::Scene;
use image::{ImageBuffer, Rgb};
use std::rc::Rc;
pub struct SimpleIntegrator {
    pub camera: Rc<Camera>,
}
impl SimpleIntegrator {
    pub fn new(camera: Rc<Camera>) -> SimpleIntegrator {
        SimpleIntegrator { camera }
    }
}
impl Integrator for SimpleIntegrator {
    fn render(&self, scene: &Scene) {
        let viewport = self.camera.get_viewport();
        let mut image_buffer = ImageBuffer::new(viewport.x, viewport.y);
        for x in 0..viewport.x {
            for y in 0..viewport.y {
                let ray = self.camera.generate_ray(Vector2::new(x, y));
                let closest_hit = scene.intersect(&ray);
                let color = (closest_hit * 255.0) as u8;
                image_buffer.put_pixel(
                    x,
                    y,
                    Rgb {
                        data: [color, color, color],
                    },
                );
            }
        }
        self.camera.write_image(&image_buffer);
    }
}
