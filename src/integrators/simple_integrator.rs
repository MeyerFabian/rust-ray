use cgmath::{Vector2, Vector3};
use core::camera::Camera;
use core::integrator::Integrator;
use core::math::Ray;
use core::scene::Scene;
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use std::time::{Duration, Instant};

pub struct SimpleIntegrator {
    pub camera: Box<Camera>,
}

impl SimpleIntegrator {
    pub fn new(camera: Box<Camera>) -> SimpleIntegrator {
        SimpleIntegrator { camera }
    }
}

impl Integrator for SimpleIntegrator {
    fn render(&self, scene: &Scene) {
        let viewport = self.camera.get_viewport();
        let now = Instant::now();

        let image_triple: Vec<_> = (0..viewport.x)
            .into_par_iter()
            .flat_map(|x| {
                (0..viewport.y)
                    .into_par_iter()
                    .map(|y| {
                        let ray = self.camera.generate_ray(Vector2::new(x, y));
                        let closest_hit = scene.intersect(&ray);
                        let mut color: u8 = 255;
                        if let Some(t) = closest_hit {
                            color = (t * 255.0) as u8;
                        }
                        (
                            x as u32,
                            y as u32,
                            Rgb {
                                data: [color, color, color],
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let new_now = Instant::now();
        println!("{:?}", new_now.duration_since(now));

        let mut image_buffer = ImageBuffer::new(viewport.x, viewport.y);
        image_triple
            .into_iter()
            .for_each(|(x, y, col)| image_buffer.put_pixel(x, y, col));
        self.camera.write_image(&image_buffer);
    }
}
