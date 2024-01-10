use super::math::{Ray, Vec3};
use super::scene::Scene;
use cgmath::{ElementWise, InnerSpace, Zero};
use rand::Rng;

const MAX_DEPTH: u32 = 10;
const RUSSIAN_ROULETTE_PROB: f32 = 0.8;

pub fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    let mut color = Vec3::zero();
    let mut rng = rand::thread_rng();

    if depth >= MAX_DEPTH {
        return color;
    }

    if rng.gen::<f32>() > RUSSIAN_ROULETTE_PROB {
        return color;
    }

    let hit = scene.intersect(ray);
    if hit.is_none() {
        return color;
    }

    let hit = hit.unwrap();
    color += hit.material.emission();

    let scatter_result = hit.material.scatter(ray, hit.p, hit.normal);
    if scatter_result.is_none() {
        return color;
    }

    let scatter_result = scatter_result.unwrap();
    let cos_theta = scatter_result.ray.direction.dot(hit.normal);
    if cos_theta <= 0.0 {
        return color;
    }
    let income = cos_theta
        * hit
            .material
            .bxdf(ray, &scatter_result.ray, hit.p, hit.normal)
            .mul_element_wise(trace(&scatter_result.ray, scene, depth + 1))
        / scatter_result.pdf;
    color += income / RUSSIAN_ROULETTE_PROB;

    return color;
}
