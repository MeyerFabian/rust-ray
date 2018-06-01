use cgmath::Vector3;
use image;

use core::shape::Shape;
use core::scene::Scene;
use core::camera::Camera;
use core::light::Light;

use shapes::sphere::Sphere;
use cameras::persp_camera::PerspectiveCamera;
use lights::point::PointLight;
pub fn run() {
    println!("API initialized.");
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(Box::new(Sphere::new(Vector3::new(0.0,0.0,0.0),0.5)));
    shapes.push(Box::new(Sphere::new(Vector3::new(0.0,0.0,0.0),0.5)));

    let mut lights: Vec<Box<Light>> = Vec::new();
    lights.push(Box::new(PointLight::new(Vector3::new(0.0,0.0,0.0))));

    let scene = Scene::new(shapes,lights);
    scene.intersect();

    let camera = PerspectiveCamera::new(Vector3::new(0.0,0.0,0.0));

    let buffer: Vec<u8> = [100;800*600*3].to_vec(); // Generate the image date
    // Save the buffer as "image.png"
    image::save_buffer("image.png", &buffer, 800, 600, image::RGB(8)).unwrap()
}
