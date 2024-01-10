use super::material::Material;
use super::math::{Ray, Vec3};
use super::scene::Scene;
use cgmath::{ElementWise, InnerSpace, Zero};
use rand::Rng;
use std::f32::consts::PI;

const MAX_DEPTH: u32 = 5;

fn build_local_coordinate_system(normal: Vec3) -> (Vec3, Vec3, Vec3) {
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

pub fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth >= MAX_DEPTH {
        return Vec3::zero();
    }

    if let Some(hit) = scene.intersect(ray) {
        match hit.material {
            Material::Emissive(color) => color,
            Material::Lambertian(color) => {
                if hit.normal.dot(ray.direction) > 0.0 {
                    return Vec3::zero();
                }
                let sampled_direction = random_in_hemisphere_cosine_weighted();
                let (u, v, w) = build_local_coordinate_system(hit.normal);
                let new_direction = u.mul_element_wise(sampled_direction.x)
                    + v.mul_element_wise(sampled_direction.y)
                    + w.mul_element_wise(sampled_direction.z);
                let new_ray = Ray {
                    origin: hit.p,
                    direction: new_direction,
                };
                color.mul_element_wise(trace(&new_ray, scene, depth + 1))
            }
            Material::Metallic(color, fuzz) => {
                // ... MCPT for metallic material
                // need mathematics equation instruction
                // raise exception for now
                panic!("Metallic material is not implemented yet!");
            }
            Material::Dielectric(refract_index) => {
                // ... MCPT for dielectric material
                // need mathematics equation instruction
                // raise exception for now
                panic!("Dielectric material is not implemented yet!");
            }
        }
    } else {
        Vec3::zero()
    }
}
