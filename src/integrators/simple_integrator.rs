use std::rc::Rc;
use core::camera::Camera;
use core::scene::Scene;
use core::integrator::Integrator;

pub struct SimpleIntegrator{
    pub camera: Rc<Camera>,
}
impl SimpleIntegrator{
    pub fn new(camera: Rc<Camera>) -> SimpleIntegrator{
        SimpleIntegrator{camera}
    }
}
impl Integrator for SimpleIntegrator{
    fn render(&self, scene: Rc<Scene>){
        self.camera.generate_rays();
        scene.intersect();
    }
}
