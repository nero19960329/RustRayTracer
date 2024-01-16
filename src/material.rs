use super::math::{reflect, spherical_to_world, Point, Ray, Vec3};
use cgmath::{InnerSpace, Zero};
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
        if (ray_out.direction - reflected).magnitude2() < 1e-3 {
            Vec3::new(1.0, 1.0, 1.0) / ray_out.direction.dot(normal)
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
