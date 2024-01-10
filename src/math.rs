use cgmath::{InnerSpace, Point3, Vector3};

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

pub fn vec3_approx_eq(v1: Vec3, v2: Vec3, epsilon: f32) -> bool {
    (v1 - v2).magnitude() < epsilon
}

pub fn point_approx_eq(p1: Point, p2: Point, epsilon: f32) -> bool {
    (p1 - p2).magnitude() < epsilon
}
