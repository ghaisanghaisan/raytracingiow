use std::{
    fs::File,
    io::{BufWriter, Write},
};

mod color;
mod ray;
mod vec3;

use color::write_color;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = vec3::dot(r.direction(), r.direction());
    let b = -2.0 * vec3::dot(r.direction(), oc);
    let c = vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let c_center = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(c_center, 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - c_center);

        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);

    //println!("{a}");
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.0, 0.0, 1.0) * a
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let file = File::create("output.ppm").unwrap();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "P3\n {} {}\n 255\n", image_width, image_height).unwrap();

    for y in 0..image_height {
        println!("Rendering {} / {} rows", y + 1, image_height);
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_v * y as f64) + (pixel_delta_u * x as f64);

            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let c = ray_color(&r);

            write_color(&mut writer, &c);
        }
    }

    writer.flush().unwrap();

    println!("Done!");
}
