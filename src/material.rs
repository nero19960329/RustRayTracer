use super::math::{Point, Ray, Vec3};
use cgmath::{ElementWise, InnerSpace, Zero};
use rand::Rng;
use std::f32::consts::{FRAC_1_PI, PI};

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

fn random_in_hemisphere_cosine_weighted() -> Vec3 {
    let mut rng = rand::thread_rng();
    let u: f32 = rng.gen();
    let v: f32 = rng.gen();
    let theta = (1.0 - u).sqrt().acos();
    let phi = 2.0 * PI * v;
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    Vec3::new(x, y, z)
}

pub struct ScatterResult {
    pub ray: Ray,
    pub pdf: f32,
}

impl ScatterResult {
    pub fn new(ray: Ray, pdf: f32) -> Self {
        Self { ray, pdf }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_point: Point, normal: Vec3) -> Option<ScatterResult>;

    fn bxdf(&self, ray_in: &Ray, ray_out: &Ray, hit_point: Point, normal: Vec3) -> Vec3;
    fn emission(&self) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_point: Point, normal: Vec3) -> Option<ScatterResult> {
        let sampled_direction = random_in_hemisphere_cosine_weighted();
        let (u, v, w) = local_coordinate_system(normal);
        let new_direction = u.mul_element_wise(sampled_direction.x)
            + v.mul_element_wise(sampled_direction.y)
            + w.mul_element_wise(sampled_direction.z);
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
