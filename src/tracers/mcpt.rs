use super::super::math::{Ray, Vec3D};
use super::super::sampler::Sampler;
use super::super::scene::Scene;
use super::tracer::Tracer;
use super::utils::{connect, generate_camera_vertices, PathVertex};
use cgmath::Zero;
use serde::Deserialize;

pub struct MonteCarloPathTracer {
    min_depth: usize,
    max_depth: usize,
}

#[derive(Deserialize)]
pub struct MonteCarloPathTracerConfig {
    pub min_depth: usize,
    pub max_depth: usize,
}

impl Tracer for MonteCarloPathTracer {
    fn trace(&mut self, ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Vec3D {
        let camera_vertices =
            generate_camera_vertices(ray, scene, sampler, self.min_depth, self.max_depth);
        let light_vertices: Vec<PathVertex> = Vec::new();

        let camera_vertex_count = camera_vertices.len();
        let light_vertex_count = light_vertices.len();

        let mut color = Vec3D::zero();
        for t in 1..(camera_vertex_count + 1) {
            for s in 0..(light_vertex_count + 1) {
                let depth = (s + t) as i32 - 2;
                if (s == 1 && t == 1) || depth < 0 || depth > self.max_depth as i32 {
                    continue;
                }

                color += connect(scene, &camera_vertices, &light_vertices, s, t, sampler);
            }
        }

        color
    }
}

impl MonteCarloPathTracerConfig {
    pub fn to_tracer(&self) -> MonteCarloPathTracer {
        MonteCarloPathTracer {
            min_depth: self.min_depth,
            max_depth: self.max_depth,
        }
    }
}
