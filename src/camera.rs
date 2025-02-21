use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    utility,
    vec3::{Color, Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub sample_per_pixel: i32,
    pub max_depth: i32,
    pub image_height: i32,
    pixel_samples_scale: f64,
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
    pub fn size_of_image(&self) -> i32 {
        self.image_width * self.image_height
    }
    pub fn new(image_width: i32, aspect_ratio: f64, sample_per_pixel: i32, max_depth: i32) -> Self {
        let mut ret = Self::default();
        ret.image_width = image_width;
        ret.aspect_ratio = aspect_ratio;
        ret.sample_per_pixel = sample_per_pixel;
        ret.max_depth = max_depth;

        ret
    }
    pub fn render(&mut self, world: &HittableList) {
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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += Camera::ray_color(&r, self.max_depth, world)
                }

                write_color(&mut writer, &(pixel_color * self.pixel_samples_scale));
            }
        }
        writer.flush().unwrap();
    }
    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let offset = self.sample_square();
        let sample_point = self.pixel00_loc
            + (self.pixel_delta_v * (y as f64 + offset.y()))
            + (self.pixel_delta_u * (x as f64 + offset.x()));

        let ray_origin = self.camera_center;
        let ray_direction = sample_point - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let x_off = utility::random_double() - 0.5;
        let y_off = utility::random_double() - 0.5;
        Vec3::new(x_off, y_off, 0.0)
    }
    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as i32).max(1);
        self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f64;
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

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            //let dir = Vec3::random_on_hemisphere(&rec.normal);
            //let dir = rec.normal + Vec3::random_unit_vector();

            let mut scattered: Ray = Ray::default();
            let mut attenuation: Color = Color::default();
            if rec.mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Camera::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
