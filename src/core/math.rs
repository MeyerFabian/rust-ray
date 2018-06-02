use cgmath::Vector3;
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}
impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { origin, direction }
    }
}
pub fn solve_quadr_eq(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let det = b * b - 4.0 * a * c;
    if det < 0.0 {
        return None;
    }
    let d = det.sqrt();
    let q = -0.5 * (b + b.signum() * d);
    let mut t1 = q / a;
    let mut t2 = c / q;
    if t1 > t2 {
        let a = t2;
        t2 = t1;
        t1 = a;
    }
    Some((t1, t2))
}
