use super::super::math::Ray;
use super::super::object::HitRecord;
use super::mesh::{Mesh, MeshConfig};
use super::plane::{Plane, PlaneConfig};
use super::quadrilateral::{Quadrilateral, QuadrilateralConfig};
use super::sphere::{Sphere, SphereConfig};
use super::triangle::{Triangle, TriangleConfig};
use serde::Deserialize;

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
    Quadrilateral(Quadrilateral),
    Mesh(Mesh),
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
    pub fn to_shape(&self) -> Shape {
        match self {
            ShapeConfig::Sphere(config) => Shape::Sphere(config.to_instance()),
            ShapeConfig::Plane(config) => Shape::Plane(config.to_instance()),
            ShapeConfig::Triangle(config) => Shape::Triangle(config.to_instance()),
            ShapeConfig::Quadrilateral(config) => Shape::Quadrilateral(config.to_instance()),
            ShapeConfig::Mesh(config) => Shape::Mesh(config.to_instance()),
        }
    }
}

impl Shape {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match *self {
            Shape::Sphere(ref sphere) => sphere.intersect(ray, t_min, t_max),
            Shape::Plane(ref plane) => plane.intersect(ray, t_min, t_max),
            Shape::Triangle(ref triangle) => triangle.intersect(ray, t_min, t_max),
            Shape::Quadrilateral(ref quadrilateral) => quadrilateral.intersect(ray, t_min, t_max),
            Shape::Mesh(ref mesh) => mesh.intersect(ray, t_min, t_max),
        }
    }
}
