use crate::vec3::Point3;

#[derive(Debug, Default)]
pub struct Ray {
    origin: Point3,
    dir: Point3,
}

impl Ray {
    pub fn new(o: Point3, d: Point3) -> Self {
        Self { origin: o, dir: d }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn direction(&self) -> Point3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}
