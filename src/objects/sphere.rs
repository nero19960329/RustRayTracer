use super::super::material::{Material, MaterialConfig};
use super::super::math::{transform_point3, Matrix4D, Matrix4DConfig, Point3D, Point3DConfig, Ray};
use super::common::HitRecord;
use cgmath::InnerSpace;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

#[derive(Deserialize)]
pub struct SphereConfig {
    pub center: Point3DConfig,
    pub radius: f64,
    pub material: MaterialConfig,
    pub transform: Option<Matrix4DConfig>,
}

impl Sphere {
    #[allow(dead_code)]
    fn intersect_analytic(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = oc.dot(ray.direction);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord {
            t: root,
            p: point,
            normal: normal,
            material: Arc::clone(&self.material),
        })
    }

    fn intersect_geometric(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let l = self.center - ray.origin;
        let t_ca = l.dot(ray.direction);

        let d2 = l.magnitude2() - t_ca * t_ca;
        if d2 < 0.0 || d2 > self.radius * self.radius {
            return None;
        }

        let t_hc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = t_ca - t_hc;
        let mut t1 = t_ca + t_hc;

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        if t0 < t_min {
            t0 = t1;
            if t0 < t_min {
                return None;
            }
        }

        if t0 > t_max {
            return None;
        }

        let point = ray.at(t0);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord {
            t: t0,
            p: point,
            normal: normal,
            material: Arc::clone(&self.material),
        })
    }

    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        return self.intersect_geometric(ray, t_min, t_max);
    }

    pub fn transform(&mut self, transform: &Matrix4D) -> Self {
        Sphere {
            center: transform_point3(*transform, self.center),
            radius: self.radius,
            material: Arc::clone(&self.material),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        material::MockMaterial,
        math::{point_approx_eq, vec3_approx_eq, Vec3D},
    };
    use approx::assert_abs_diff_eq;
    use rand::Rng;

    #[test]
    fn test_sphere_intersect() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let center = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );
            let radius = rng.gen_range(0.1..10.0);
            let sphere = Sphere {
                center: center,
                radius: radius,
                material: Arc::new(MockMaterial {}),
            };
            let p1 = Ray {
                origin: center,
                direction: Vec3D::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize(),
            }
            .at(rng.gen_range(0.0..radius * 2.0));
            let p2 = Ray {
                origin: center,
                direction: Vec3D::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize(),
            }
            .at(rng.gen_range(0.0..radius * 2.0));
            let ray = Ray {
                origin: p1,
                direction: (p2 - p1).normalize(),
            };
            let hit_analytic = sphere.intersect_analytic(&ray, 0.0, 100.0);
            let hit_geometric = sphere.intersect_geometric(&ray, 0.0, 100.0);
            if hit_analytic.is_none() || hit_geometric.is_none() {
                assert!(hit_analytic.is_none() && hit_geometric.is_none());
            } else {
                let hit_analytic = hit_analytic.unwrap();
                let hit_geometric = hit_geometric.unwrap();
                assert_abs_diff_eq!(hit_analytic.t, hit_geometric.t, epsilon = 1e-6);
                point_approx_eq(hit_analytic.p, hit_geometric.p, 1e-6);
                vec3_approx_eq(hit_analytic.normal, hit_geometric.normal, 1e-6);
            }
        }
    }
}
