use super::math::{fresnel, reflect, refract, spherical_to_world, Point, Ray, Vec3};
use cgmath::{Array, InnerSpace, Zero};
use log::warn;
use rand::Rng;
use std::f32::consts::{FRAC_1_PI, PI};
use std::fmt::Debug;

pub struct ScatterResult {
    pub ray: Ray,
    pub pdf: f32,
}

impl ScatterResult {
    pub fn new(ray: Ray, pdf: f32) -> Self {
        Self { ray, pdf }
    }
}

pub trait Material: Sync + Send + Debug {
    fn scatter(&self, ray_in: &Ray, hit_point: Point, normal: Vec3) -> Option<ScatterResult>;

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, hit_point: Point, normal: Vec3) -> Vec3;
    fn emission(&self) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Clone)]
pub struct Emissive {
    pub color: Vec3,
}

impl Material for Emissive {
    fn scatter(&self, _: &Ray, _: Point, _: Vec3) -> Option<ScatterResult> {
        None
    }

    fn bxdf(&self, _: &Ray, _: &Ray, _: Point, _: Vec3) -> Vec3 {
        Vec3::zero()
    }

    fn emission(&self) -> Vec3 {
        self.color
    }
}

impl Debug for Emissive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emissive")
            .field("color", &self.color)
            .finish()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_point: Point, normal: Vec3) -> Option<ScatterResult> {
        let mut rng = rand::thread_rng();
        let u: f32 = rng.gen();
        let v: f32 = rng.gen();
        let theta = (1.0 - u).sqrt().acos();
        let phi = 2.0 * PI * v;

        let new_direction = spherical_to_world(theta, phi, normal);
        let new_ray = Ray {
            origin: hit_point,
            direction: new_direction,
        };
        let pdf = new_direction.dot(normal) * FRAC_1_PI;
        Some(ScatterResult::new(new_ray, pdf))
    }

    fn bxdf(&self, _: &Ray, _: &Ray, _: Point, _: Vec3) -> Vec3 {
        self.albedo * FRAC_1_PI
    }
}

impl Debug for Lambertian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lambertian")
            .field("albedo", &self.albedo)
            .finish()
    }
}

#[derive(Clone)]
pub struct PhongSpecular {
    pub specular: Vec3,
    pub shininess: f32,
}

impl Material for PhongSpecular {
    fn scatter(&self, ray_in: &Ray, hit_point: Point, normal: Vec3) -> Option<ScatterResult> {
        let reflected = reflect(ray_in.direction, normal);
        let mut rng = rand::thread_rng();
        let u: f32 = rng.gen();
        let v: f32 = rng.gen();
        let theta = u.powf(1.0 / (self.shininess + 1.0)).acos();
        let phi = 2.0 * PI * v;

        let new_direction = spherical_to_world(theta, phi, reflected);
        let new_ray = Ray {
            origin: hit_point,
            direction: new_direction,
        };
        let pdf = new_direction.dot(reflected).powf(self.shininess)
            * (self.shininess + 1.0)
            * FRAC_1_PI
            * 0.5;
        Some(ScatterResult::new(new_ray, pdf))
    }

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, _: Point, normal: Vec3) -> Vec3 {
        let reflected = reflect(ray_in.direction, normal);
        let cos_theta = reflected.dot(ray_out.direction);
        if cos_theta < 0.0 {
            Vec3::zero()
        } else {
            self.specular
                * (self.shininess + 2.0)
                * FRAC_1_PI
                * 0.5
                * cos_theta.powf(self.shininess)
        }
    }
}

impl Debug for PhongSpecular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhongSpecular")
            .field("specular", &self.specular)
            .field("shininess", &self.shininess)
            .finish()
    }
}

#[derive(Clone)]
pub struct IdealReflector {}

impl Material for IdealReflector {
    fn scatter(&self, ray_in: &Ray, hit_point: Point, normal: Vec3) -> Option<ScatterResult> {
        let reflected = reflect(ray_in.direction, normal);
        let new_ray = Ray {
            origin: hit_point,
            direction: reflected,
        };
        Some(ScatterResult::new(new_ray, 1.0))
    }

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, _: Point, normal: Vec3) -> Vec3 {
        let reflected = reflect(ray_in.direction, normal);
        let cos_theta = ray_out.direction.dot(normal);
        if cos_theta > 1e-3 && (ray_out.direction - reflected).magnitude2() < 1e-3 {
            Vec3::new(1.0, 1.0, 1.0) / cos_theta
        } else {
            Vec3::zero()
        }
    }
}

impl Debug for IdealReflector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdealReflector").finish()
    }
}

#[derive(Clone)]
pub struct IdealDielectric {
    pub ior: f32, // index of refraction
}

impl Material for IdealDielectric {
    fn scatter(&self, ray_in: &Ray, hit_point: Point, normal: Vec3) -> Option<ScatterResult> {
        let mut outward_normal = normal; // normal pointing out of the surface

        // check if ray is inside the object
        let mut eta_i = 1.0;
        let mut eta_t = self.ior;
        if ray_in.direction.dot(normal) > 0.0 {
            eta_i = self.ior;
            eta_t = 1.0;
            outward_normal = -normal;
        }
        let eta = eta_i / eta_t;

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(outward_normal);
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();
        let reflectance = fresnel(cos_theta, eta_i, eta_t);
        if reflectance > 1.0 + 1e-3 {
            panic!("reflectance > 1.0");
        }
        if r < reflectance {
            // reflect
            let reflected = reflect(unit_direction, outward_normal);
            let new_ray = Ray {
                origin: hit_point,
                direction: reflected,
            };
            return Some(ScatterResult::new(new_ray, reflectance));
        } else {
            // refract
            let refracted = refract(unit_direction, outward_normal, eta);
            if refracted.is_none() {
                return None;
            }
            let refracted = refracted.unwrap();
            let new_ray = Ray {
                origin: hit_point,
                direction: refracted,
            };
            return Some(ScatterResult::new(new_ray, 1.0 - reflectance));
        }
    }

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, _: Point, normal: Vec3) -> Vec3 {
        let mut outward_normal = normal; // normal pointing out of the surface

        // check if ray is inside the object
        let mut eta_i = 1.0;
        let mut eta_t = self.ior;
        if ray_in.direction.dot(normal) > 0.0 {
            eta_i = self.ior;
            eta_t = 1.0;
            outward_normal = -normal;
        }
        let eta = eta_i / eta_t;

        let cos_theta_i = ray_in.direction.dot(normal).abs();
        let cos_theta_t = ray_out.direction.dot(normal).abs();
        if cos_theta_t < 1e-3 {
            return Vec3::zero();
        }

        let reflectance = fresnel(cos_theta_i, eta_i, eta_t);
        let transmittance = 1.0 - reflectance;

        let reflect_dir = reflect(ray_in.direction, outward_normal);
        let refract_dir = refract(ray_in.direction, outward_normal, eta).unwrap_or(Vec3::zero());

        let mut bxdf = Vec3::zero();
        if (reflect_dir - ray_out.direction).magnitude2() < 1e-3 {
            bxdf = Vec3::new(1.0, 1.0, 1.0) * reflectance / cos_theta_t;
        } else if (refract_dir - ray_out.direction).magnitude2() < 1e-3 {
            bxdf = Vec3::new(1.0, 1.0, 1.0) * transmittance / (cos_theta_t * eta * eta);
        }

        if !bxdf.is_finite() {
            warn!(
                "bxdf not finite, reflectance: {}, transmittance: {}, cos_theta_t: {}, eta: {}",
                reflectance, transmittance, cos_theta_t, eta,
            )
        }

        bxdf
    }
}

impl Debug for IdealDielectric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdealDielectric")
            .field("ior", &self.ior)
            .finish()
    }
}
