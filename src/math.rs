use cgmath::{ElementWise, InnerSpace, Matrix4, Point2, Point3, Vector3, Vector4};
use serde::Deserialize;

pub type Vec3D = Vector3<f64>;
pub type Vec4D = Vector4<f64>;
pub type Point2U = Point2<u32>;
pub type Point3D = Point3<f64>;
pub type Matrix4D = Matrix4<f64>;

#[derive(Deserialize)]
pub struct Vec3DConfig {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3DConfig {
    pub fn to_vec3(&self) -> Vec3D {
        Vec3D::new(self.x, self.y, self.z)
    }
}

#[derive(Deserialize)]
pub struct Point3DConfig {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3DConfig {
    pub fn to_point(&self) -> Point3D {
        Point3D::new(self.x, self.y, self.z)
    }
}

#[derive(Deserialize)]
pub struct Matrix4DConfig {
    m11: f64,
    m12: f64,
    m13: f64,
    m14: f64,
    m21: f64,
    m22: f64,
    m23: f64,
    m24: f64,
    m31: f64,
    m32: f64,
    m33: f64,
    m34: f64,
    m41: f64,
    m42: f64,
    m43: f64,
    m44: f64,
}

impl Matrix4DConfig {
    pub fn to_matrix(&self) -> Matrix4D {
        Matrix4D::new(
            self.m11, self.m12, self.m13, self.m14, self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34, self.m41, self.m42, self.m43, self.m44,
        )
    }
}

const EYE_MATRIX4D: Matrix4D = Matrix4D {
    x: Vec4D {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    },
    y: Vec4D {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    },
    z: Vec4D {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 0.0,
    },
    w: Vec4D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    },
};

pub fn unwrap_matrix4d_config_to_matrix4d(config: Option<&Matrix4DConfig>) -> Matrix4D {
    match config {
        Some(config) => config.to_matrix(),
        None => EYE_MATRIX4D,
    }
}

pub fn point3_to_vec4(p: Point3D) -> Vec4D {
    Vec4D::new(p.x, p.y, p.z, 1.0)
}

pub fn vec4_to_point3(v: Vec4D) -> Point3D {
    assert!(v.w != 0.0);
    Point3D::new(v.x / v.w, v.y / v.w, v.z / v.w)
}

pub fn transform_point3(m: Matrix4D, p: Point3D) -> Point3D {
    vec4_to_point3(m * point3_to_vec4(p))
}

pub fn transform_vec3(m: Matrix4D, v: Vec3D) -> Vec3D {
    let u = m * Vec4D::new(v.x, v.y, v.z, 0.0);
    Vec3D::new(u.x, u.y, u.z)
}

pub fn max_component(v: Vec3D) -> f64 {
    v.x.max(v.y).max(v.z)
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }
}

pub fn reflect(v: Vec3D, n: Vec3D) -> Vec3D {
    v - n * 2.0 * v.dot(n)
}

pub fn refract(v: Vec3D, n: Vec3D, eta: f64) -> Option<Vec3D> {
    let cos_theta1 = (-v).dot(n);
    let sin2_theta1 = 1.0 - cos_theta1 * cos_theta1;
    let sin2_theta2 = sin2_theta1 * eta * eta;
    if sin2_theta2 > 1.0 {
        return None;
    }
    let cos_theta2 = (1.0 - sin2_theta2).sqrt();
    Some(eta * v + (eta * cos_theta1 - cos_theta2) * n)
}

pub fn fresnel(cos_i: f64, eta_i: f64, eta_t: f64) -> f64 {
    // eta_i is the index of refraction of the medium the ray is coming from
    // eta_t is the index of refraction of the medium the ray is entering
    let sin2_t = (eta_i / eta_t) * (eta_i / eta_t) * (1.0 - cos_i * cos_i);
    if sin2_t > 1.0 {
        // total internal reflection
        return 1.0;
    }
    let cos_t = (1.0 - sin2_t).sqrt();
    let r_ortho = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
    let r_parallel = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
    (r_ortho * r_ortho + r_parallel * r_parallel) / 2.0
}

fn local_coordinate_system(normal: Vec3D) -> (Vec3D, Vec3D, Vec3D) {
    let w = normal;
    let a = if w.x.abs() > 0.9 {
        Vec3D::new(0.0, 1.0, 0.0)
    } else {
        Vec3D::new(1.0, 0.0, 0.0)
    };
    let u = w.cross(a).normalize();
    let v = w.cross(u).normalize();
    (u, v, w)
}

pub fn spherical_to_world(theta: f64, phi: f64, normal: Vec3D) -> Vec3D {
    let (u, v, w) = local_coordinate_system(normal);
    u.mul_element_wise(theta.sin() * phi.cos())
        + v.mul_element_wise(theta.sin() * phi.sin())
        + w.mul_element_wise(theta.cos())
}

#[cfg(test)]
pub fn vec3_approx_eq(v1: Vec3D, v2: Vec3D, epsilon: f64) -> bool {
    (v1 - v2).magnitude() < epsilon
}

#[cfg(test)]
pub fn point_approx_eq(p1: Point3D, p2: Point3D, epsilon: f64) -> bool {
    (p1 - p2).magnitude() < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use rand::Rng;
    use std::f64::consts::PI;

    #[test]
    fn test_ray_at() {
        let ray = Ray {
            origin: Point3D::new(0.0, 0.0, 0.0),
            direction: Vec3D::new(1.0, 0.0, 0.0),
        };
        let t = 1.0;
        let p = ray.at(t);
        assert_eq!(p, Point3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_reflect() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = Vec3D::new(rng.gen(), rng.gen(), rng.gen()).normalize();
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

    #[test]
    fn test_refract() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = Vec3D::new(rng.gen(), rng.gen(), rng.gen()).normalize();
            let v = spherical_to_world(
                rng.gen_range(0.5 * PI..PI), // hemisphere
                rng.gen_range(0.0..2.0 * PI),
                n,
            );
            assert!(v.dot(n) <= 0.0);
            let eta = rng.gen_range(0.5..2.0);
            let r = refract(v, n, eta);
            // snell's law
            let cos_theta1 = (-v).dot(n);
            let sin_theta1 = (1.0 - cos_theta1 * cos_theta1).sqrt();
            let sin_theta2 = sin_theta1 * eta;
            if sin_theta2 > 1.0 {
                assert!(r.is_none());
            } else {
                let r = r.unwrap();
                assert!(r.dot(n) <= 0.0);
                let cos_theta2 = r.dot(-n);
                assert_abs_diff_eq!(
                    sin_theta2,
                    (1.0 - cos_theta2 * cos_theta2).sqrt(),
                    epsilon = 1e-3
                );
            }
        }
    }

    #[test]
    fn test_fresnel() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = Vec3D::new(rng.gen(), rng.gen(), rng.gen()).normalize();
            let v = spherical_to_world(
                rng.gen_range(0.5 * PI..PI), // hemisphere
                rng.gen_range(0.0..2.0 * PI),
                n,
            );
            assert!(v.dot(n) <= 0.0);
            let eta_i = rng.gen_range(0.5..2.0);
            let eta_t = rng.gen_range(0.5..2.0);
            let cos_theta_i = (-v).dot(n);
            let cos_theta_t = refract(v, n, eta_i / eta_t).map(|r| r.dot(-n));
            if cos_theta_t.is_none() {
                assert_abs_diff_eq!(fresnel(cos_theta_i, eta_i, eta_t), 1.0, epsilon = 1e-3);
            } else {
                let cos_theta_t = cos_theta_t.unwrap();
                assert_abs_diff_eq!(
                    fresnel(cos_theta_i, eta_i, eta_t),
                    fresnel(cos_theta_t, eta_t, eta_i),
                    epsilon = 1e-3
                );
            }
        }
    }

    #[test]
    fn test_local_coordinate_system() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = Vec3D::new(rng.gen(), rng.gen(), rng.gen()).normalize();
            let (u, v, w) = local_coordinate_system(n);
            assert_abs_diff_eq!(u.dot(v), 0.0, epsilon = 1e-6);
            assert_abs_diff_eq!(v.dot(w), 0.0, epsilon = 1e-6);
            assert_abs_diff_eq!(w.dot(u), 0.0, epsilon = 1e-6);
            assert_abs_diff_eq!(u.magnitude(), 1.0, epsilon = 1e-6);
            assert_abs_diff_eq!(v.magnitude(), 1.0, epsilon = 1e-6);
            assert_abs_diff_eq!(w.magnitude(), 1.0, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_spherical_to_world() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = Vec3D::new(rng.gen(), rng.gen(), rng.gen()).normalize();
            let theta = rng.gen_range(0.0..PI);
            let phi = rng.gen_range(0.0..2.0 * PI);
            let v = spherical_to_world(theta, phi, n);
            assert_abs_diff_eq!(v.magnitude(), 1.0, epsilon = 1e-6);
            assert_abs_diff_eq!(v.dot(n), theta.cos(), epsilon = 1e-6);
        }
    }
}
