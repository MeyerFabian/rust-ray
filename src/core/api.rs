use cgmath::Vector3;
use image;
use std::rc::Rc;

use core::shape::Shape;
use core::scene::Scene;
use core::light::Light;
use core::material::Material;
use core::integrator::Integrator;

use shapes::sphere::Sphere;
use cameras::persp_camera::PerspectiveCamera;
use lights::point::PointLight;
use materials::plastic::Plastic;
use integrators::simple_integrator::SimpleIntegrator;

pub fn run() {
    println!("API initialized.");


    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(Box::new(Sphere::new(Vector3::new(0.0,0.0,0.0),0.5)));
    shapes.push(Box::new(Sphere::new(Vector3::new(0.0,0.0,0.0),0.5)));

    let mut materials: Vec<Box<Material>> = Vec::new();
    materials.push(Box::new(Plastic::new()));
    materials.push(Box::new(Plastic::new()));

    let mut lights: Vec<Box<Light>> = Vec::new();
    lights.push(Box::new(PointLight::new(Vector3::new(0.0,0.0,0.0))));

    let scene = Scene::new(shapes,materials,lights);
    let camera = PerspectiveCamera::new(Vector3::new(0.0,0.0,0.0));

    let integrator = SimpleIntegrator::new(Rc::new(camera));
    integrator.render(Rc::new(scene));

    let buffer: Vec<u8> = [100;800*600*3].to_vec(); // Generate the image date
    // Save the buffer as "image.png"
    image::save_buffer("image.png", &buffer, 800, 600, image::RGB(8)).unwrap()
}
