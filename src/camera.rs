use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::{
    color::write_color_to_string,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    utility,
    vec3::{Color, Point3, Vec3},
};

use rayon::prelude::*;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub image_height: i32,
    pub vfov: f64,
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
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
    ) -> Self {
        let mut ret = Self::default();
        ret.image_width = image_width;
        ret.aspect_ratio = aspect_ratio;
        ret.samples_per_pixel = samples_per_pixel;
        ret.max_depth = max_depth;
        ret.vfov = vfov;

        ret
    }
    pub fn size_of_image(&self) -> i32 {
        self.image_width * self.image_height
    }
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        let file = File::create("output.ppm").unwrap();

        let mut writer = BufWriter::new(file);
        //let writer = Arc::new(Mutex::new(writer));

        // Write the PPM header once at the start
        {
            //let mut writer = writer.lock().unwrap();
            writeln!(
                writer,
                "P3\n{} {}\n255",
                self.image_width, self.image_height
            )
            .unwrap();
        }

        let progress = Arc::new(AtomicUsize::new(0));
        let total_rows = self.image_height;

        let rows: Vec<_> = (0..self.image_height)
            .into_par_iter()
            .map(|y| {
                let pixel_colors: Vec<_> = (0..self.image_width)
                    .into_par_iter()
                    .map(|x| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(x, y);
                            pixel_color += Camera::ray_color(&r, self.max_depth, world);
                        }
                        pixel_color * self.pixel_samples_scale
                    })
                    .collect();

                let completed = progress.fetch_add(1, Ordering::Relaxed) + 1;
                println!("Rendered {}/{} rows", completed, total_rows);

                // Convert row to string after collecting all pixels
                let mut pixel_str = String::with_capacity(self.image_width as usize * 12);
                for color in pixel_colors {
                    write_color_to_string(&mut pixel_str, &color);
                }
                pixel_str
            })
            .collect();

        // Write rows in correct order
        //let mut writer = writer.lock().unwrap();
        for row_data in rows {
            writeln!(writer, "{}", row_data).unwrap();
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

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let x_off = utility::random_double() - 0.5;
        let y_off = utility::random_double() - 0.5;
        Vec3::new(x_off, y_off, 0.0)
    }
    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as i32).max(1);
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.focal_length = 1.0;

        let theta = utility::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();

        self.viewport_height = 2.0 * h * self.focal_length;
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

    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
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
        let unit_direction = Vec3::unit_vector(&r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
