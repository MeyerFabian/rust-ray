use core::scene::Scene;

pub trait Integrator{
    fn render(&self,&Scene);
}
