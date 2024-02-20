use super::super::common::HitRecord;
use super::super::math::{
    transform_point3, unwrap_matrix4d_config_to_matrix4d, Matrix4D, Matrix4DConfig, Point3D,
    Point3DConfig, Ray, Vec3D,
};
use super::super::sampler::Sampler;
use super::shape::{SampleResult, Shape};
use cgmath::InnerSpace;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Point3D; 3],
}

#[derive(Deserialize)]
pub struct TriangleConfig {
    pub vertices: [Point3DConfig; 3],
    pub transform: Option<Matrix4DConfig>,
}

pub fn triangle_intersect(
    v0: Point3D,
    v1: Point3D,
    v2: Point3D,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<(f64, f64, f64)> {
    // Moller-Trumbore algorithm
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let h = ray.direction.cross(e2);
    let a = e1.dot(h);

    if a.abs() < 1e-6 {
        return None; // ray is parallel to triangle
    }

    let f = 1.0 / a;
    let s = ray.origin - v0;
    let u = f * s.dot(h);

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = s.cross(e1);
    let v = f * ray.direction.dot(q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * e2.dot(q);
    if t > t_min && t < t_max {
        return Some((t, u, v));
    } else {
        return None;
    }
}

fn uniform_triangle_barycentric(u: f64, v: f64) -> (f64, f64) {
    let su0 = u.sqrt();
    let b0 = 1.0 - su0;
    let b1 = v * su0;
    (b0, b1)
}

fn barycentric_to_point(b0: f64, b1: f64, v0: Point3D, v1: Point3D, v2: Point3D) -> Point3D {
    Point3D::new(
        b0 * v0.x + b1 * v1.x + (1.0 - b0 - b1) * v2.x,
        b0 * v0.y + b1 * v1.y + (1.0 - b0 - b1) * v2.y,
        b0 * v0.z + b1 * v1.z + (1.0 - b0 - b1) * v2.z,
    )
}

pub fn triangle_sample(v0: Point3D, v1: Point3D, v2: Point3D, u: f64, v: f64) -> Point3D {
    let (b0, b1) = uniform_triangle_barycentric(u, v);
    barycentric_to_point(b0, b1, v0, v1, v2)
}

pub fn triangle_area(v0: Point3D, v1: Point3D, v2: Point3D) -> f64 {
    0.5 * (v1 - v0).cross(v2 - v0).magnitude()
}

pub fn triangle_normal(v0: Point3D, v1: Point3D, v2: Point3D) -> Vec3D {
    (v1 - v0).cross(v2 - v0).normalize()
}

#[allow(dead_code)]
pub fn in_triangle(p: Point3D, v0: Point3D, v1: Point3D, v2: Point3D) -> bool {
    let e0 = v1 - v0;
    let e1 = v2 - v0;
    let e2 = p - v0;
    let d00 = e0.dot(e0);
    let d01 = e0.dot(e1);
    let d11 = e1.dot(e1);
    let d20 = e2.dot(e0);
    let d21 = e2.dot(e1);
    let denom = d00 * d11 - d01 * d01;
    let u = (d11 * d20 - d01 * d21) / denom;
    let v = (d00 * d21 - d01 * d20) / denom;
    u >= 0.0 && v >= 0.0 && u + v <= 1.0
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (t, _u, _v) = match triangle_intersect(
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
            ray,
            t_min,
            t_max,
        ) {
            Some((t, u, v)) => (t, u, v),
            None => return None,
        };

        let p = ray.at(t);
        let normal = (self.vertices[1] - self.vertices[0])
            .cross(self.vertices[2] - self.vertices[0])
            .normalize();
        return Some(HitRecord {
            t: t,
            p: p,
            normal: normal,
            shape: Some(self as &dyn Shape),
            object: None,
        });
    }

    fn transform(&self, transform: &Matrix4D) -> Arc<dyn Shape> {
        Arc::new(Triangle {
            vertices: [
                transform_point3(*transform, self.vertices[0]),
                transform_point3(*transform, self.vertices[1]),
                transform_point3(*transform, self.vertices[2]),
            ],
        })
    }

    fn sample(&self, sampler: &mut dyn Sampler) -> SampleResult {
        let (u, v) = sampler.get_2d();
        let p = triangle_sample(self.vertices[0], self.vertices[1], self.vertices[2], u, v);
        let normal = triangle_normal(self.vertices[0], self.vertices[1], self.vertices[2]);
        let area = triangle_area(self.vertices[0], self.vertices[1], self.vertices[2]);

        SampleResult {
            p: p,
            normal: normal,
            pdf: 1.0 / area,
        }
    }
}

impl TriangleConfig {
    pub fn to_shape(&self) -> Arc<dyn Shape> {
        Triangle {
            vertices: [
                self.vertices[0].to_point(),
                self.vertices[1].to_point(),
                self.vertices[2].to_point(),
            ],
        }
        .transform(&unwrap_matrix4d_config_to_matrix4d(self.transform.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{vec3_approx_eq, Vec3D};
    use approx::assert_abs_diff_eq;
    use rand::Rng;

    #[test]
    fn test_triangle_intersect() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let v0 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let v1 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let v2 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let triangle = Triangle {
                vertices: [v0, v1, v2],
            };
            let p1 = Ray {
                origin: v0,
                direction: Vec3D::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize(),
            }
            .at(rng.gen_range(0.0..10.0));
            let p2 = Ray {
                origin: v0,
                direction: Vec3D::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize(),
            }
            .at(rng.gen_range(0.0..10.0));
            let ray = Ray {
                origin: p1,
                direction: (p2 - p1).normalize(),
            };
            let hit = triangle.intersect(&ray, 0.0, 100.0);
            if hit.is_none() {
                assert!(hit.is_none());
            } else {
                let hit = hit.unwrap();
                let normal = (v1 - v0).cross(v2 - v0).normalize();
                assert_abs_diff_eq!((hit.p - v0).dot(normal), 0.0, epsilon = 1e-3);
                vec3_approx_eq(hit.normal, normal, 1e-3);
                assert!(in_triangle(hit.p, v0, v1, v2));
            }
        }
    }

    #[test]
    fn test_triangle_sample() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let v0 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let v1 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let v2 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let mut count = 0;
            for _ in 0..1000 {
                let p = triangle_sample(v0, v1, v2, rng.gen(), rng.gen());
                if in_triangle(p, v0, v1, v2) {
                    count += 1;
                }
            }
            assert!(count == 1000);
        }
    }
}
