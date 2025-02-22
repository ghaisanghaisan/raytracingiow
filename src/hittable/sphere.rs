use std::sync::Arc;

use crate::{
    interval::Interval,
    material::{Lambertian, Material},
    vec3::{dot, Point3},
};

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new<M: Material + 'static + Send + Sync>(c: Point3, r: f64, m: M) -> Self {
        Self {
            center: c,
            radius: r.max(0.0),
            mat: Arc::new(m),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        let sqrtd = discriminant.sqrt();

        if discriminant < 0.0 {
            return false;
        }

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.t = root;
        record.p = r.at(root);
        let outward_normal = (record.p - self.center) / self.radius;

        record.set_face_normal(r, outward_normal);
        record.mat = self.mat.clone();

        true
    }
}
