use std::rc::Rc;
use core::scene::Scene;

pub trait Integrator{
    fn render(&self,Rc<Scene>);
}
