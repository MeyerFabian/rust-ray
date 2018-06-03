use cgmath::{InnerSpace, Vector2};
use core::camera::Camera;
use core::integrator::Integrator;
use core::math::{reflect, Ray};
use core::scene::Scene;
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use std::time::Instant;

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
                        let closest_hit = scene.intersectP(&ray);
                        let mut color = Rgb { data: [0, 0, 0] };
                        if let Some((i, rayhit)) = closest_hit {
                            let total_phong_contrib = scene
                                .lights
                                .iter()
                                .map(|light| {
                                    let hit_to_light_source =
                                        (light.position() - rayhit.position).normalize();
                                    let mut phong = hit_to_light_source.dot(rayhit.normal);
                                    let shadow_hit = scene.intersect(&Ray::new(
                                        rayhit.position + 0.001 * hit_to_light_source,
                                        hit_to_light_source,
                                    ));
                                    if let Some(shadow_contrib) = shadow_hit {
                                        phong = phong * shadow_contrib / 2.0;
                                    }

                                    phong
                                })
                                .sum::<f32>();

                            let mut to_rgb = scene.primitives[i].material.color().map(|color| {
                                (color as f32) * total_phong_contrib.min(1.0).max(0.0)
                            });
                            let max_distance = 10.0;
                            let mut normal = rayhit.normal;
                            let mut direction = ray.direction;
                            let mut position = rayhit.position;
                            let mut distance = rayhit.distance;
                            while (distance < max_distance) {
                                let bounce = scene.intersectP(&Ray::new(
                                    position + 0.001 * reflect(direction, normal),
                                    reflect(direction, normal),
                                ));
                                if let Some((j, refl_hit)) = bounce {
                                    let sq_distance = refl_hit.distance * refl_hit.distance;
                                    to_rgb += scene.primitives[j].material.color().map(|color| {
                                        (color as f32)
                                            * (total_phong_contrib / sq_distance / 30.0)
                                                .min(1.0)
                                                .max(0.0)
                                    });
                                }
                                break;
                            }

                            color.data = [(to_rgb.x) as u8, (to_rgb.y) as u8, (to_rgb.z) as u8];
                        }
                        (x as u32, (viewport.y - 1 - y) as u32, color)
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
