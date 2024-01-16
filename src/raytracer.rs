use super::math::{Ray, Vec3};
use super::scene::Scene;
use cgmath::{Array, ElementWise, InnerSpace, Zero};
use log::warn;
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
    if scatter_result.pdf <= 1e-3 {
        return color;
    }
    let cos_theta = scatter_result.ray.direction.dot(hit.normal);
    if cos_theta <= 0.0 {
        return color;
    }
    let bxdf = hit
        .material
        .bxdf(ray, &scatter_result.ray, hit.p, hit.normal);
    if !bxdf.is_finite() {
        warn!("bxdf not finite, hit.material: {:?}", hit.material);
    }
    let income = cos_theta * bxdf.mul_element_wise(trace(&scatter_result.ray, scene, depth + 1))
        / scatter_result.pdf;
    if !income.is_finite() {
        warn!("income not finite");
    }
    color += income / RUSSIAN_ROULETTE_PROB;

    return color;
}
