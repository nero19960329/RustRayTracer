use super::material::Material;
use super::math::{Ray, Vec3};
use super::scene::Scene;
use cgmath::{ElementWise, InnerSpace};
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

fn random_in_hemisphere(normal: Vec3) -> Vec3 { // cosine-weighted hemisphere sampling
    let mut rng = rand::thread_rng();
    let u: f32 = rng.gen();
    let v: f32 = rng.gen();
    let theta = (1.0 - u).sqrt().acos();
    let phi = 2.0 * PI * v;
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    let (local_u, local_v, local_w) = build_local_coordinate_system(normal);
    local_u * x + local_v * y + local_w * z
}

pub fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth >= MAX_DEPTH {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // info!("DEPTH: {}, ray.origin: {:?}, ray.direction: {:?}", depth, ray.origin, ray.direction);

    if let Some(hit) = scene.intersect(ray) {
        // info!("DEPTH: {}, hit.p: {:?}, hit.normal: {:?}", depth, hit.p, hit.normal);

        match hit.material {
            Material::Emissive(color) => {
                // info!("DEPTH: {}, HIT EMISSIVE", depth);
                color
            }
            Material::Lambertian(color) => {
                // info!("DEPTH: {}, HIT LAMBERTIAN", depth);
                if hit.normal.dot(ray.direction) > 0.0 {
                    return Vec3::new(0.0, 0.0, 0.0);
                }
                let new_direction = random_in_hemisphere(hit.normal);
                let new_ray = Ray {
                    origin: hit.p + new_direction * 1e-3,
                    direction: new_direction,
                };
                // info!("DEPTH: {}, new_ray.origin: {:?}, new_ray.direction: {:?}", depth, new_ray.origin, new_ray.direction);
                let new_color = color.mul_element_wise(trace(&new_ray, scene, depth + 1));
                // info!("DEPTH: {}, new_color: {:?}", depth, new_color);
                new_color
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
        Vec3::new(0.0, 0.0, 0.0)
    }
}
