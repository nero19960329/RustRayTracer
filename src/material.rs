use super::math::{
    fresnel, reflect, refract, spherical_to_world, Point3D, Ray, Vec3D, Vec3DConfig,
};
use cgmath::{Array, InnerSpace, Zero};
use log::warn;
use rand::Rng;
use serde::Deserialize;
use std::f64::consts::{FRAC_1_PI, PI};
use std::fmt::Debug;
use std::sync::Arc;

pub struct ScatterResult {
    pub ray: Ray,
    pub pdf: f64,
}

impl ScatterResult {
    pub fn new(ray: Ray, pdf: f64) -> Self {
        Self { ray, pdf }
    }
}

pub trait Material: Sync + Send + Debug {
    fn scatter(&self, ray_in: &Ray, hit_point: Point3D, normal: Vec3D) -> Option<ScatterResult>;

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, hit_point: Point3D, normal: Vec3D) -> Vec3D;
    fn emission(&self) -> Vec3D {
        Vec3D::zero()
    }
}

#[derive(Debug, Clone)]
pub struct MockMaterial;

impl Material for MockMaterial {
    fn scatter(&self, _: &Ray, _: Point3D, _: Vec3D) -> Option<ScatterResult> {
        None
    }

    fn bxdf(&self, _: &Ray, _: &Ray, _: Point3D, _: Vec3D) -> Vec3D {
        Vec3D::zero()
    }
}

#[derive(Debug, Clone)]
pub struct Emissive {
    pub color: Vec3D,
}

impl Material for Emissive {
    fn scatter(&self, _: &Ray, _: Point3D, _: Vec3D) -> Option<ScatterResult> {
        None
    }

    fn bxdf(&self, _: &Ray, _: &Ray, _: Point3D, _: Vec3D) -> Vec3D {
        Vec3D::zero()
    }

    fn emission(&self) -> Vec3D {
        self.color
    }
}

#[derive(Deserialize)]
pub struct EmissiveConfig {
    pub color: Vec3DConfig,
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Vec3D,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_point: Point3D, normal: Vec3D) -> Option<ScatterResult> {
        let mut rng = rand::thread_rng();
        let u: f64 = rng.gen();
        let v: f64 = rng.gen();
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

    fn bxdf(&self, _: &Ray, _: &Ray, _: Point3D, _: Vec3D) -> Vec3D {
        self.albedo * FRAC_1_PI
    }
}

#[derive(Deserialize)]
pub struct LambertianConfig {
    pub albedo: Vec3DConfig,
}

#[derive(Debug, Clone)]
pub struct PhongSpecular {
    pub specular: Vec3D,
    pub shininess: f64,
}

impl Material for PhongSpecular {
    fn scatter(&self, ray_in: &Ray, hit_point: Point3D, normal: Vec3D) -> Option<ScatterResult> {
        let reflected = reflect(ray_in.direction, normal);
        let mut rng = rand::thread_rng();
        let u: f64 = rng.gen();
        let v: f64 = rng.gen();
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

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, _: Point3D, normal: Vec3D) -> Vec3D {
        let reflected = reflect(ray_in.direction, normal);
        let cos_theta = reflected.dot(ray_out.direction);
        if cos_theta < 0.0 {
            Vec3D::zero()
        } else {
            self.specular
                * (self.shininess + 2.0)
                * FRAC_1_PI
                * 0.5
                * cos_theta.powf(self.shininess)
        }
    }
}

#[derive(Deserialize)]
pub struct PhongSpecularConfig {
    pub specular: Vec3DConfig,
    pub shininess: f64,
}

#[derive(Debug, Clone)]
pub struct IdealReflector {}

#[derive(Deserialize)]
pub struct IdealReflectorConfig {}

impl Material for IdealReflector {
    fn scatter(&self, ray_in: &Ray, hit_point: Point3D, normal: Vec3D) -> Option<ScatterResult> {
        let reflected = reflect(ray_in.direction, normal);
        let new_ray = Ray {
            origin: hit_point,
            direction: reflected,
        };
        Some(ScatterResult::new(new_ray, 1.0))
    }

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, _: Point3D, normal: Vec3D) -> Vec3D {
        let reflected = reflect(ray_in.direction, normal);
        let cos_theta = ray_out.direction.dot(normal);
        if cos_theta > 1e-6 && (ray_out.direction - reflected).magnitude2() < 1e-6 {
            Vec3D::new(1.0, 1.0, 1.0) / cos_theta
        } else {
            Vec3D::zero()
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdealDielectric {
    pub ior: f64, // index of refraction
}

#[derive(Deserialize)]
pub struct IdealDielectricConfig {
    pub ior: f64,
}

impl Material for IdealDielectric {
    fn scatter(&self, ray_in: &Ray, hit_point: Point3D, normal: Vec3D) -> Option<ScatterResult> {
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
        let r: f64 = rng.gen();
        let reflectance = fresnel(cos_theta, eta_i, eta_t);
        if reflectance > 1.0 {
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

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, _: Point3D, normal: Vec3D) -> Vec3D {
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
        if cos_theta_t < 1e-6 {
            return Vec3D::zero();
        }

        let reflectance = fresnel(cos_theta_i, eta_i, eta_t);
        let transmittance = 1.0 - reflectance;

        let reflect_dir = reflect(ray_in.direction, outward_normal);
        let refract_dir = refract(ray_in.direction, outward_normal, eta).unwrap_or(Vec3D::zero());

        let mut bxdf = Vec3D::zero();
        if (reflect_dir - ray_out.direction).magnitude2() < 1e-6 {
            bxdf = Vec3D::new(1.0, 1.0, 1.0) * reflectance / cos_theta_t;
        } else if (refract_dir - ray_out.direction).magnitude2() < 1e-6 {
            bxdf = Vec3D::new(1.0, 1.0, 1.0) * transmittance / (cos_theta_t * eta * eta);
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

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum MaterialConfig {
    Emissive(EmissiveConfig),
    Lambertian(LambertianConfig),
    PhongSpecular(PhongSpecularConfig),
    IdealReflector(IdealReflectorConfig),
    IdealDielectric(IdealDielectricConfig),
}

impl MaterialConfig {
    pub fn to_material(&self) -> Arc<dyn Material> {
        match self {
            MaterialConfig::Emissive(config) => Arc::new(Emissive {
                color: config.color.to_vec3(),
            }),
            MaterialConfig::Lambertian(config) => Arc::new(Lambertian {
                albedo: config.albedo.to_vec3(),
            }),
            MaterialConfig::PhongSpecular(config) => Arc::new(PhongSpecular {
                specular: config.specular.to_vec3(),
                shininess: config.shininess,
            }),
            MaterialConfig::IdealReflector(_) => Arc::new(IdealReflector {}),
            MaterialConfig::IdealDielectric(config) => {
                Arc::new(IdealDielectric { ior: config.ior })
            }
        }
    }
}
