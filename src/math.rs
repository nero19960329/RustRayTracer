use cgmath::{Point3, Vector3};

pub type Vec3 = Vector3<f32>;
pub type Point = Point3<f32>;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}
