use std::sync::Arc;

use crate::{
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub mod sphere;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            t: f64::default(),
            front_face: bool::default(),
            mat: Arc::new(Lambertian::default()),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;

        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from<T: Hittable + 'static>(objects: Vec<Arc<T>>) -> Self {
        Self {
            objects: objects
                .into_iter()
                .map(|o| o as Arc<dyn Hittable>)
                .collect(),
        }
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            let new_t = Interval::new(ray_t.min, closest_so_far);
            if obj.hit(r, new_t, record) {
                hit_anything = true;
                closest_so_far = record.t;
            }
        }

        hit_anything
    }
}

impl Clone for HittableList {
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.clone(),
        }
    }
}
