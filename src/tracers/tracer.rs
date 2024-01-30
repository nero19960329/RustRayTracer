use super::super::math::{Ray, Vec3D};
use super::super::sampler::Sampler;
use super::super::scene::Scene;
use super::mcpt::MonteCarloPathTracerConfig;
use serde::Deserialize;

pub trait Tracer {
    fn trace(&mut self, ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Vec3D;
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum TracerConfig {
    #[serde(rename = "mcpt")]
    MonteCarloPathTracer(MonteCarloPathTracerConfig),
}

impl TracerConfig {
    pub fn to_tracer(&self) -> Box<dyn Tracer> {
        match self {
            TracerConfig::MonteCarloPathTracer(config) => Box::new(config.to_tracer()),
        }
    }
}
