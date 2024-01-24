use super::super::math::Ray;
use super::common::HitRecord;
use super::plane::{Plane, PlaneConfig};
use super::sphere::{Sphere, SphereConfig};
use serde::Deserialize;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Plane(PlaneConfig),
}

impl ObjectConfig {
    pub fn to_object(&self) -> Object {
        match self {
            ObjectConfig::Sphere(config) => Object::Sphere(config.to_instance()),
            ObjectConfig::Plane(config) => Object::Plane(config.to_instance()),
        }
    }
}

impl Object {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray, t_min, t_max),
            Object::Plane(ref plane) => plane.intersect(ray, t_min, t_max),
        }
    }
}
