use super::material::Material;
use super::math::{Ray, Vec3};
use super::scene::Scene;
use cgmath::ElementWise;
use rand::Rng;
use std::f32::consts::PI;

const MAX_DEPTH: u32 = 5;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a: f32 = rng.gen_range(0.0..2.0 * PI);
    let z: f32 = rng.gen_range(0.0..1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth >= MAX_DEPTH {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = scene.intersect(ray) {
        match hit.material {
            Material::Emissive(color) => color,
            Material::Lambertian(color) => {
                let target = hit.p + hit.normal + random_in_unit_sphere();
                let new_ray = Ray {
                    origin: hit.p,
                    direction: target - hit.p,
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
        Vec3::new(0.0, 0.0, 0.0)
    }
}
