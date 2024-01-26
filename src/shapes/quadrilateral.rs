use super::super::math::{
    transform_point3, unwrap_matrix4d_config_to_matrix4d, Matrix4D, Matrix4DConfig, Point3D,
    Point3DConfig, Ray,
};
use super::super::object::HitRecord;
use cgmath::InnerSpace;
use log::debug;
use serde::Deserialize;

#[derive(Debug)]
pub struct Quadrilateral {
    pub vertices: [Point3D; 4],
}

#[derive(Deserialize)]
pub struct QuadrilateralConfig {
    pub vertices: [Point3DConfig; 4],
    pub transform: Option<Matrix4DConfig>,
}

pub fn are_points_coplanar(v0: Point3D, v1: Point3D, v2: Point3D, v3: Point3D) -> bool {
    let e01 = v1 - v0;
    let e02 = v2 - v0;
    let e03 = v3 - v0;
    let n1 = e01.cross(e02);
    let n2 = e02.cross(e03);
    // cosine of the angle between n1 and n2
    let cos = n1.dot(n2) / (n1.magnitude() * n2.magnitude());
    debug!("cosine of the angle between n1 and n2: {}", cos);
    (cos.abs() - 1.0).abs() < 1e-3
}

pub fn is_quadrilateral_convex(v0: Point3D, v1: Point3D, v2: Point3D, v3: Point3D) -> bool {
    if !are_points_coplanar(v0, v1, v2, v3) {
        return false;
    }

    let e01 = v1 - v0;
    let e12 = v2 - v1;
    let e23 = v3 - v2;
    let e30 = v0 - v3;

    let cross1 = e01.cross(e12);
    let cross2 = e12.cross(e23);
    let cross3 = e23.cross(e30);
    let cross4 = e30.cross(e01);

    cross1.dot(cross2) >= 0.0
        && cross2.dot(cross3) >= 0.0
        && cross3.dot(cross4) >= 0.0
        && cross4.dot(cross1) >= 0.0
}

pub fn quadrilateral_intersect(
    v0: Point3D,
    v1: Point3D,
    v2: Point3D,
    v3: Point3D,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<(f64, f64, f64, f64)> {
    // reject rays using the barycentric coordinates of
    // the intersection point with respect to t
    let e01 = v1 - v0;
    let e03 = v3 - v0;
    let p = ray.direction.cross(e03);
    let det = e01.dot(p);
    if det.abs() < 1e-6 {
        return None;
    }

    let inv_det = 1.0 / det;
    let t = ray.origin - v0;
    let alpha = t.dot(p) * inv_det;
    if alpha < 0.0 {
        return None;
    }

    let q = t.cross(e01);
    let beta = ray.direction.dot(q) * inv_det;
    if beta < 0.0 {
        return None;
    }

    // reject rays using the barycentric coordinates of
    // intersection point with respect to t'
    if (alpha + beta) > 1.0 {
        let e23 = v3 - v2;
        let e21 = v1 - v2;
        let p_prime = ray.direction.cross(e21);
        let det_prime = e23.dot(p_prime);
        if det_prime.abs() < 1e-6 {
            return None;
        }

        let inv_det_prime = 1.0 / det_prime;
        let t_prime = ray.origin - v2;
        let alpha_prime = t_prime.dot(p_prime) * inv_det_prime;
        if alpha_prime < 0.0 {
            return None;
        }

        let q_prime = t_prime.cross(e23);
        let beta_prime = ray.direction.dot(q_prime) * inv_det_prime;
        if beta_prime < 0.0 {
            return None;
        }
    }

    // compute the ray parameter of the intersection point
    let ray_t = e03.dot(q) * inv_det;
    if ray_t < t_min || ray_t > t_max {
        return None;
    }

    // compute the barycentric coordinates of v2
    let e02 = v2 - v0;
    let n = e01.cross(e03);
    let mut alpha11: f64 = 0.0;
    let mut beta11: f64 = 0.0;
    if n.x.abs() >= n.y.abs() && n.x.abs() >= n.z.abs() {
        alpha11 = (e02.y * e03.z - e02.z * e03.y) / n.x;
        beta11 = (e01.y * e02.z - e01.z * e02.y) / n.x;
    } else if n.y.abs() >= n.x.abs() && n.y.abs() >= n.z.abs() {
        alpha11 = (e02.z * e03.x - e02.x * e03.z) / n.y;
        beta11 = (e01.z * e02.x - e01.x * e02.z) / n.y;
    } else if n.z.abs() >= n.x.abs() && n.z.abs() >= n.y.abs() {
        alpha11 = (e02.x * e03.y - e02.y * e03.x) / n.z;
        beta11 = (e01.x * e02.y - e01.y * e02.x) / n.z;
    }

    // compute the bilinear coordinates of the intersection point
    let mut u: f64;
    let v: f64;
    if (alpha11 - 1.0).abs() < 1e-6 {
        u = alpha;
        if (beta11 - 1.0).abs() < 1e-6 {
            v = beta;
        } else {
            v = beta / (u * (beta11 - 1.0) + 1.0);
        }
    } else if (beta11 - 1.0).abs() < 1e-6 {
        v = beta;
        u = alpha / (v * (alpha11 - 1.0) + 1.0);
    } else {
        let a = 1.0 - beta11;
        let b = alpha * (beta11 - 1.0) - beta * (alpha11 - 1.0) - 1.0;
        let c = alpha;
        let discriminant = b * b - 4.0 * a * c;
        let big_q = -0.5 * (b + if b > 0.0 { 1.0 } else { -1.0 } * discriminant.sqrt());
        u = big_q / a;
        if u < 0.0 || u > 1.0 {
            u = c / big_q;
        }
        v = beta / (u * (beta11 - 1.0) + 1.0);
    }

    Some((ray_t, u * (1.0 - v), u * v, (1.0 - u) * v))
}

impl Quadrilateral {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (t, _u, _v, _w) = match quadrilateral_intersect(
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
            self.vertices[3],
            ray,
            t_min,
            t_max,
        ) {
            Some((t, u, v, w)) => (t, u, v, w),
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
        Quadrilateral {
            vertices: [
                transform_point3(*transform, self.vertices[0]),
                transform_point3(*transform, self.vertices[1]),
                transform_point3(*transform, self.vertices[2]),
                transform_point3(*transform, self.vertices[3]),
            ],
        }
    }
}

impl QuadrilateralConfig {
    pub fn to_instance(&self) -> Quadrilateral {
        Quadrilateral {
            vertices: [
                self.vertices[0].to_point(),
                self.vertices[1].to_point(),
                self.vertices[2].to_point(),
                self.vertices[3].to_point(),
            ],
        }
        .transform(&unwrap_matrix4d_config_to_matrix4d(self.transform.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::triangle::triangle_intersect;
    use approx::assert_abs_diff_eq;
    use rand::Rng;

    #[test]
    fn test_are_points_coplanar() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let v0 = Point3D::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0), 0.0);
            let v1 = Point3D::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0), 0.0);
            let v2 = Point3D::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0), 0.0);
            if rng.gen::<f32>() < 0.5 {
                let v3 = Point3D::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0), 0.0);
                assert!(are_points_coplanar(v0, v1, v2, v3));
            } else {
                let v3 = Point3D::new(
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                );
                if v3.z.abs() > 1.0 {
                    assert!(!are_points_coplanar(v0, v1, v2, v3));
                }
            }
        }
    }

    #[test]
    fn test_is_quadrilateral_convex() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let v0 = Point3D::new(-1.0, 1.0, 0.0);
            let v1 = Point3D::new(-1.0, -1.0, 0.0);
            let v2 = Point3D::new(1.0, -1.0, 0.0);
            if rng.gen::<f32>() < 0.5 {
                // convex
                let v3 = Point3D::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0);
                assert!(are_points_coplanar(v0, v1, v2, v3));
                assert!(is_quadrilateral_convex(v0, v1, v2, v3));
            } else {
                let v3 = Point3D::new(-rng.gen::<f64>(), -rng.gen::<f64>(), 0.0);
                assert!(are_points_coplanar(v0, v1, v2, v3));
                assert!(!is_quadrilateral_convex(v0, v1, v2, v3));
            }
        }
    }

    #[test]
    fn test_quadrilateral_intersection() {
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            let v0 = Point3D::new(rng.gen_range(-10.0..0.0), rng.gen_range(0.0..10.0), 0.0);
            let v1 = Point3D::new(rng.gen_range(-10.0..0.0), rng.gen_range(-10.0..0.0), 0.0);
            let v2 = Point3D::new(rng.gen_range(0.0..10.0), rng.gen_range(-10.0..0.0), 0.0);
            let v3 = Point3D::new(rng.gen_range(0.0..10.0), rng.gen_range(0.0..10.0), 0.0);
            if !is_quadrilateral_convex(v0, v1, v2, v3) {
                continue;
            }

            let p1 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..0.0),
            );
            let p2 = Point3D::new(
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
                rng.gen_range(0.0..10.0),
            );
            let ray = Ray {
                origin: p1,
                direction: (p2 - p1).normalize(),
            };
            let hit_quadrilateral = quadrilateral_intersect(v0, v1, v2, v3, &ray, 0.0, 100.0);
            let hit_triangle_0 = triangle_intersect(v0, v1, v2, &ray, 0.0, 100.0);
            let hit_triangle_1 = triangle_intersect(v0, v2, v3, &ray, 0.0, 100.0);
            let hit_triangle = match (hit_triangle_0, hit_triangle_1) {
                (Some((t0, u0, v0)), Some((t1, u1, v1))) => {
                    if t0 < t1 {
                        Some((t0, u0, v0))
                    } else {
                        Some((t1, u1, v1))
                    }
                }
                (Some((t0, u0, v0)), None) => Some((t0, u0, v0)),
                (None, Some((t1, u1, v1))) => Some((t1, u1, v1)),
                (None, None) => None,
            };

            if hit_quadrilateral.is_none() || hit_triangle.is_none() {
                assert!(hit_quadrilateral.is_none() && hit_triangle.is_none());
            } else {
                assert!(hit_quadrilateral.is_some() && hit_triangle.is_some());

                let hit_quadrilateral = hit_quadrilateral.unwrap();
                let hit_triangle = hit_triangle.unwrap();
                let hit_point_quadrilateral = ray.at(hit_quadrilateral.0);
                let hit_point_triangle = ray.at(hit_triangle.0);
                assert_abs_diff_eq!(
                    (hit_point_quadrilateral - hit_point_triangle).magnitude(),
                    0.0,
                    epsilon = 1e-6
                );
            }
        }
    }
}
