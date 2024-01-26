use super::super::math::{
    transform_point3, transform_vec3, unwrap_matrix4d_config_to_matrix4d, Matrix4D, Matrix4DConfig,
    Point3D, Point3DConfig, Ray, Vec3D, Vec3DConfig,
};
use super::super::object::HitRecord;
use cgmath::InnerSpace;
use serde::Deserialize;

#[derive(Debug)]
pub struct Plane {
    pub point: Point3D,
    pub normal: Vec3D,
}

#[derive(Deserialize)]
pub struct PlaneConfig {
    pub point: Point3DConfig,
    pub normal: Vec3DConfig,
    pub transform: Option<Matrix4DConfig>,
}

impl Plane {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denominator = self.normal.dot(ray.direction);
        if denominator.abs() < 1e-6 {
            return None;
        }

        let v = self.point - ray.origin;
        let distance = v.dot(self.normal) / denominator;
        if distance < t_min || distance > t_max {
            return None;
        }

        Some(HitRecord {
            t: distance,
            p: ray.at(distance),
            normal: self.normal,
            material: None,
        })
    }

    pub fn transform(&mut self, transform: &Matrix4D) -> Self {
        Plane {
            point: transform_point3(*transform, self.point),
            normal: transform_vec3(*transform, self.normal).normalize(),
        }
    }
}

impl PlaneConfig {
    pub fn to_instance(&self) -> Plane {
        Plane {
            point: self.point.to_point(),
            normal: self.normal.to_vec3().normalize(),
        }
        .transform(&unwrap_matrix4d_config_to_matrix4d(self.transform.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::vec3_approx_eq;
    use approx::assert_abs_diff_eq;
    use rand::Rng;

    #[test]
    fn test_plane_intersect() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let point = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let normal = Vec3D::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize();
            let plane = Plane {
                point: point,
                normal: normal,
            };
            let p1 = Ray {
                origin: point,
                direction: Vec3D::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize(),
            }
            .at(rng.gen_range(0.0..10.0));
            let p2 = Ray {
                origin: point,
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
            let hit = plane.intersect(&ray, 0.0, 100.0);
            if hit.is_none() {
                assert!(hit.is_none());
            } else {
                let hit = hit.unwrap();
                assert_abs_diff_eq!((hit.p - plane.point).dot(normal), 0.0, epsilon = 1e-6);
                vec3_approx_eq(hit.normal, normal, 1e-6);
            }
        }
    }
}
