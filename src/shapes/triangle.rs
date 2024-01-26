use super::super::math::{
    transform_point3, unwrap_matrix4d_config_to_matrix4d, Matrix4D, Matrix4DConfig, Point3D,
    Point3DConfig, Ray,
};
use super::super::object::HitRecord;
use cgmath::InnerSpace;
use serde::Deserialize;

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

impl Triangle {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
            material: None,
        });
    }

    pub fn transform(&mut self, transform: &Matrix4D) -> Self {
        Triangle {
            vertices: [
                transform_point3(*transform, self.vertices[0]),
                transform_point3(*transform, self.vertices[1]),
                transform_point3(*transform, self.vertices[2]),
            ],
        }
    }
}

impl TriangleConfig {
    pub fn to_instance(&self) -> Triangle {
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

                // test barycentric coordinates
                let denom = (v1 - v0).cross(v2 - v0).magnitude();
                let u = ((v2 - v0).cross(hit.p - v0)).magnitude() / denom;
                let v = ((v0 - v1).cross(hit.p - v1)).magnitude() / denom;
                assert!(u >= 0.0 && u <= 1.0);
                assert!(v >= 0.0 && v <= 1.0);
                assert!(u + v <= 1.0);
            }
        }
    }
}
