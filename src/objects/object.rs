use super::super::math::Ray;
use super::common::HitRecord;
use super::plane::{Plane, PlaneConfig};
use super::quadrilateral::{Quadrilateral, QuadrilateralConfig};
use super::sphere::{Sphere, SphereConfig};
use super::triangle::{Triangle, TriangleConfig};
use serde::Deserialize;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
    Quadrilateral(Quadrilateral),
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Plane(PlaneConfig),
    Triangle(TriangleConfig),
    Quadrilateral(QuadrilateralConfig),
}

impl ObjectConfig {
    pub fn to_object(&self) -> Object {
        match self {
            ObjectConfig::Sphere(config) => Object::Sphere(config.to_instance()),
            ObjectConfig::Plane(config) => Object::Plane(config.to_instance()),
            ObjectConfig::Triangle(config) => Object::Triangle(config.to_instance()),
            ObjectConfig::Quadrilateral(config) => Object::Quadrilateral(config.to_instance()),
        }
    }
}

impl Object {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray, t_min, t_max),
            Object::Plane(ref plane) => plane.intersect(ray, t_min, t_max),
            Object::Triangle(ref triangle) => triangle.intersect(ray, t_min, t_max),
            Object::Quadrilateral(ref quadrilateral) => quadrilateral.intersect(ray, t_min, t_max),
        }
    }
}
