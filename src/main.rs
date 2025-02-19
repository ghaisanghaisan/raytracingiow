use core::f64;

mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod vec3;

use camera::Camera;
use hittable::sphere::Sphere;
use hittable::HittableList;
use vec3::Point3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

    // Camera

    let mut world = HittableList::empty();
    let a = Sphere::new(Point3::new(0.5, 0.0, -1.0), 0.5);
    let b = Sphere::new(Point3::new(-0.75, 0.0, -1.5), 0.5);
    world.add(&a);
    world.add(&b);

    let camera = Camera::new(image_width, image_height);
    camera.render(&mut world);

    println!("Done!");
}
