mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod utility;
mod vec3;

use camera::Camera;
use hittable::sphere::Sphere;
use hittable::HittableList;
use vec3::Point3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Camera

    let spheres = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];
    let world = HittableList::from(spheres);

    let camera = Camera::new(image_width, aspect_ratio, 100, 10);
    camera.render(&world);

    println!("Done!");
}
