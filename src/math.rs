use cgmath::{ElementWise, InnerSpace, Point3, Vector3};

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

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(n)
}

fn local_coordinate_system(normal: Vec3) -> (Vec3, Vec3, Vec3) {
    let w = normal;
    let a = if w.x.abs() > 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u = w.cross(a).normalize();
    let v = w.cross(u).normalize();
    (u, v, w)
}

pub fn spherical_to_world(theta: f32, phi: f32, normal: Vec3) -> Vec3 {
    let (u, v, w) = local_coordinate_system(normal);
    u.mul_element_wise(theta.sin() * phi.cos())
        + v.mul_element_wise(theta.sin() * phi.sin())
        + w.mul_element_wise(theta.cos())
}

#[cfg(test)]
pub fn vec3_approx_eq(v1: Vec3, v2: Vec3, epsilon: f32) -> bool {
    (v1 - v2).magnitude() < epsilon
}

#[cfg(test)]
pub fn point_approx_eq(p1: Point, p2: Point, epsilon: f32) -> bool {
    (p1 - p2).magnitude() < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::f32::consts::PI;

    #[test]
    fn test_reflect() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = Vec3::new(rng.gen(), rng.gen(), rng.gen()).normalize();
            let v = spherical_to_world(
                rng.gen_range(0.5 * PI..PI), // hemisphere
                rng.gen_range(0.0..2.0 * PI),
                n,
            );
            assert!(v.dot(n) <= 0.0);
            let r = reflect(v, n);
            assert!(r.dot(n) >= 0.0);
            vec3_approx_eq(n, (r - v).normalize(), 1e-6);
        }
    }
}
