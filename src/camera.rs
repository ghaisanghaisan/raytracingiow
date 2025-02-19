use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    camera_center: Point3,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32) -> Self {
        let mut ret = Self::default();
        ret.image_width = image_width;
        ret.image_height = image_height;

        ret
    }
    pub fn render(mut self, world: &mut HittableList) {
        self.initialize();
        let file = File::create("output.ppm").unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(
            writer,
            "P3\n {} {}\n 255\n",
            self.image_width, self.image_height
        )
        .unwrap();
        for y in 0..self.image_height {
            println!("Rendering {} / {} rows", y + 1, self.image_height);
            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_v * y as f64)
                    + (self.pixel_delta_u * x as f64);

                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(&self.camera_center, &ray_direction);

                let c = Camera::ray_color(&r, world);

                write_color(&mut writer, &c);
            }
        }
        writer.flush().unwrap();
    }
    fn initialize(&mut self) {
        self.focal_length = 1.0;
        self.viewport_height = 2.0;
        self.viewport_width =
            self.viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.camera_center = Point3::new(0.0, 0.0, 0.0);

        self.viewport_u = Vec3::new(self.viewport_width, 0.0, 0.0);
        self.viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        self.pixel_delta_u = self.viewport_u / self.image_width as f64;
        self.pixel_delta_v = self.viewport_v / self.image_height as f64;

        self.viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, self.focal_length)
            - self.viewport_u / 2.0
            - self.viewport_v / 2.0;
        self.pixel00_loc =
            self.viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(r: &Ray, world: &mut dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }
        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
