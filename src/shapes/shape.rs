use super::super::common::HitRecord;
use super::super::math::{Matrix4D, Point3D, Ray, Vec3D};
use super::super::sampler::Sampler;
use super::mesh::MeshConfig;
use super::plane::PlaneConfig;
use super::quadrilateral::QuadrilateralConfig;
use super::sphere::SphereConfig;
use super::triangle::TriangleConfig;
use serde::Deserialize;
use std::sync::Arc;

pub struct SampleResult {
    pub p: Point3D,
    pub normal: Vec3D,
    pub pdf: f64,
}

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn transform(&self, transform: &Matrix4D) -> Arc<dyn Shape>;
    fn sample(&self, sampler: &mut dyn Sampler) -> SampleResult;
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ShapeConfig {
    Sphere(SphereConfig),
    Plane(PlaneConfig),
    Triangle(TriangleConfig),
    Quadrilateral(QuadrilateralConfig),
    Mesh(MeshConfig),
}

impl ShapeConfig {
    pub fn to_shape(&self) -> Arc<dyn Shape> {
        match self {
            ShapeConfig::Sphere(config) => config.to_shape(),
            ShapeConfig::Plane(config) => config.to_shape(),
            ShapeConfig::Triangle(config) => config.to_shape(),
            ShapeConfig::Quadrilateral(config) => config.to_shape(),
            ShapeConfig::Mesh(config) => config.to_shape(),
        }
    }
}
