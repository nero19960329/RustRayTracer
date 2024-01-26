use super::material::Material;
use super::math::{max_component, Point3D, Ray, Vec3D};
use super::sampler::Sampler;
use super::scene::Scene;
use cgmath::{Array, ElementWise, InnerSpace, Zero};
use log::warn;
use std::sync::Arc;

const MIN_DEPTH: u32 = 2;
const MAX_DEPTH: u32 = 6;

struct PathVertex {
    position: Point3D,
    normal: Vec3D,
    beta: Vec3D, // throughput, means cumulative contribution of the path
    material: Option<Arc<dyn Material>>,
}

fn generate_camera_vertices(
    camera_ray: &Ray,
    scene: &Scene,
    sampler: &mut dyn Sampler,
) -> Vec<PathVertex> {
    let mut path: Vec<PathVertex> = Vec::new();
    let mut beta = Vec3D::new(1.0, 1.0, 1.0);
    let mut ray = camera_ray.clone();

    let path_vertex = PathVertex {
        position: ray.origin,
        normal: Vec3D::zero(),
        beta: beta,
        material: None,
    };
    path.push(path_vertex);

    for depth in 0..MAX_DEPTH {
        let hit = scene.intersect(&ray);
        if hit.is_none() {
            break;
        }

        let hit = hit.unwrap();
        let material = &hit.material.unwrap();

        let path_vertex = PathVertex {
            position: hit.p,
            normal: hit.normal,
            beta: beta,
            material: Some(Arc::clone(material)),
        };
        path.push(path_vertex);

        if material.emission().magnitude() > 1e-6 {
            break;
        }

        let continue_prob = if depth > MIN_DEPTH {
            max_component(beta).min(1.0)
        } else {
            1.0
        };
        if sampler.get_1d() > continue_prob {
            break;
        }
        beta /= continue_prob;

        let scatter_result = material.scatter(&ray, hit.p, hit.normal, sampler);
        if scatter_result.is_none() {
            break;
        }

        let scatter_result = scatter_result.unwrap();
        if scatter_result.pdf <= 1e-6 {
            break;
        }

        let cos_theta = scatter_result.ray.direction.dot(hit.normal).abs();
        let bxdf = material.bxdf(&ray, &scatter_result.ray, hit.p, hit.normal);
        if !bxdf.is_finite() {
            warn!("bxdf not finite, hit.material: {:?}", material);
        }

        beta = beta.mul_element_wise(cos_theta * bxdf / scatter_result.pdf);
        if !beta.is_finite() {
            warn!("beta not finite");
        }

        ray = scatter_result.ray.clone();
    }

    return path;
}

fn emissive_material(material: &Option<Arc<dyn Material>>) -> bool {
    if material.is_none() {
        return false;
    }
    let material = material.as_ref().unwrap();
    material.emission().magnitude() > 1e-6
}

fn connect(
    scene: &Scene,
    camera_vertices: &Vec<PathVertex>,
    light_vertices: &Vec<PathVertex>,
    s: usize,
    t: usize,
    sampler: &mut dyn Sampler,
) -> Vec3D {
    let mut color = Vec3D::zero();

    if t > 1 && s > 0 && emissive_material(&camera_vertices[t - 1].material) {
        return color;
    }

    if s == 0 {
        let vertex = &camera_vertices[t - 1];
        if emissive_material(&vertex.material) {
            color += vertex
                .beta
                .mul_element_wise(vertex.material.as_ref().unwrap().emission());
        }
    } else {
        panic!("not implemented");
    }

    color
}

pub fn trace(ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Vec3D {
    let camera_vertices = generate_camera_vertices(ray, scene, sampler);
    let light_vertices: Vec<PathVertex> = Vec::new();

    let camera_vertex_count = camera_vertices.len();
    let light_vertex_count = light_vertices.len();

    let mut color = Vec3D::zero();
    for t in 1..(camera_vertex_count + 1) {
        for s in 0..(light_vertex_count + 1) {
            let depth = (s + t) as i32 - 2;
            if (s == 1 && t == 1) || depth < 0 || depth > MAX_DEPTH as i32 {
                continue;
            }

            color += connect(scene, &camera_vertices, &light_vertices, s, t, sampler);
        }
    }

    color
}
