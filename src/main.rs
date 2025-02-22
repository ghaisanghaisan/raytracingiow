mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod utility;
mod vec3;

use std::time::Instant;
use std::{f64::consts::PI, sync::Arc};

use camera::Camera;
use hittable::sphere::Sphere;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Metal};
use vec3::{Color, Point3};

fn three_spheres_scene() -> HittableList {
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.50);
    let material_right = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0);

    let spheres = vec![
        Arc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Arc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.2),
            0.5,
            material_center,
        )),
        Arc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )),
        Arc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.4,
            material_bubble,
        )),
        Arc::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ];
    HittableList::from(spheres)
}

fn two_spheres_scene() -> HittableList {
    let material_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));

    let r = (PI / 4.0).cos();

    let spheres = vec![
        Arc::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, material_left)),
        Arc::new(Sphere::new(Point3::new(r, 0.0, -1.0), r, material_right)),
    ];
    HittableList::from(spheres)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let world = three_spheres_scene();
    //let world = Arc::new(world);

    let start = Instant::now();
    let mut camera = Camera::new(image_width, aspect_ratio, 100, 50, 90.0);
    camera.render(&world);
    let duration = start.elapsed();

    println!("Done!");
    println!(
        "Rendered {} rays at {}x{} resolution in: {:?}",
        camera.size_of_image() * camera.samples_per_pixel,
        camera.image_width,
        camera.image_height,
        duration
    );
}
