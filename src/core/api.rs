extern crate image;

use shapes::sphere::Sphere;
use core::shape::Shape;
pub fn run() {
    println!("API initialized.");
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(Box::new(Sphere::new(0.5)));
    shapes.push(Box::new(Sphere::new(0.5)));

    for shape in shapes {
        shape.intersect();
    }

    let buffer: Vec<u8> = [100;800*600*3].to_vec(); // Generate the image date
    // Save the buffer as "image.png"
    image::save_buffer("image.png", &buffer, 800, 600, image::RGB(8)).unwrap()
}
