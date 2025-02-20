mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod utility;
mod vec3;

use camera::Camera;
use hittable::sphere::Sphere;
use hittable::HittableList;
use material::{Lambertian, Metal};
use vec3::{Color, Point3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1280;

    // Camera
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2));

    let spheres = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.2),
            0.5,
            material_center,
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ];
    let world = HittableList::from(spheres);

    let camera = Camera::new(image_width, aspect_ratio, 100, 50);
    camera.render(&world);

    println!("Done!");
}
