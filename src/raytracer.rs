use super::math::{Ray, Vec3D};
use super::sampler::Sampler;
use super::scene::Scene;
use cgmath::{Array, ElementWise, InnerSpace, Zero};
use log::warn;

const MIN_DEPTH: u32 = 3;
const RUSSIAN_ROULETTE_PROB: f64 = 0.8;

pub fn trace(ray: &Ray, scene: &Scene, depth: u32, sampler: &mut dyn Sampler) -> Vec3D {
    let mut color = Vec3D::zero();

    let p = {
        if depth < MIN_DEPTH {
            1.0
        } else {
            RUSSIAN_ROULETTE_PROB
        }
    };

    if sampler.get_1d() > p {
        return color;
    }

    let hit = scene.intersect(ray);
    if hit.is_none() {
        return color;
    }

    let hit = hit.unwrap();
    color += hit.material.emission();

    let scatter_result = hit.material.scatter(ray, hit.p, hit.normal, sampler);
    if scatter_result.is_none() {
        return color;
    }

    let scatter_result = scatter_result.unwrap();
    if scatter_result.pdf <= 1e-6 {
        return color;
    }
    let cos_theta = scatter_result.ray.direction.dot(hit.normal).abs();
    let bxdf = hit
        .material
        .bxdf(ray, &scatter_result.ray, hit.p, hit.normal);
    if !bxdf.is_finite() {
        warn!("bxdf not finite, hit.material: {:?}", hit.material);
    }
    let income = cos_theta
        * bxdf.mul_element_wise(trace(&scatter_result.ray, scene, depth + 1, sampler))
        / scatter_result.pdf;
    if !income.is_finite() {
        warn!("income not finite");
    }
    color += income / p;

    return color;
}
