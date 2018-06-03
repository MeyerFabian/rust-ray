use cgmath::{Vector2, Vector3};
use image;
use std::rc::Rc;

use core::integrator::Integrator;
use core::light::Light;
use core::primitive::Primitive;
use core::scene::Scene;

use cameras::persp_camera::PerspectiveCamera;
use integrators::simple_integrator::SimpleIntegrator;
use lights::point::PointLight;
use materials::plastic::Plastic;
use shapes::sphere::Sphere;

pub fn run() {
    let mut primitives: Vec<Primitive> = Vec::new();

    primitives.push(Primitive::new(
        Box::new(Sphere::new(Vector3::new(0.6, 0.0, -0.75), 0.5)),
        Box::new(Plastic::new()),
    ));

    primitives.push(Primitive::new(
        Box::new(Sphere::new(Vector3::new(0.0, 0.1, -0.75), 0.25)),
        Box::new(Plastic::new()),
    ));

    primitives.push(Primitive::new(
        Box::new(Sphere::new(Vector3::new(0.0, -500.2, 0.0), 500.0)),
        Box::new(Plastic::new()),
    ));

    let mut lights: Vec<Box<Light>> = Vec::new();
    lights.push(Box::new(PointLight::new(Vector3::new(0.0, 1.0, 1.0))));

    let scene = Scene::new(primitives, lights);
    let camera = PerspectiveCamera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector2::new(1920, 1080),
    );

    let integrator = SimpleIntegrator::new(Box::new(camera));
    integrator.render(&scene);
}
