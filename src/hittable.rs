use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub mod sphere;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
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

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
}

pub struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from(objects: Vec<&'a dyn Hittable>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, obj: &'a dyn Hittable) {
        self.objects.push(obj);
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in self.objects.clone() {
            let new_t = Interval::new(ray_t.min, closest_so_far);
            if obj.hit(r, new_t, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}
