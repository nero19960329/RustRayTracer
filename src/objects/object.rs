use super::super::math::{unwrap_matrix4d_config_to_matrix4d, Ray};
use super::common::HitRecord;
use super::plane::{Plane, PlaneConfig};
use super::sphere::{Sphere, SphereConfig};
use cgmath::InnerSpace;
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
            ObjectConfig::Sphere(config) => Object::Sphere(
                Sphere {
                    center: config.center.to_point(),
                    radius: config.radius,
                    material: config.material.to_material(),
                }
                .transform(&unwrap_matrix4d_config_to_matrix4d(
                    config.transform.as_ref(),
                )),
            ),
            ObjectConfig::Plane(config) => Object::Plane(
                Plane {
                    point: config.point.to_point(),
                    normal: config.normal.to_vec3().normalize(),
                    material: config.material.to_material(),
                }
                .transform(&unwrap_matrix4d_config_to_matrix4d(
                    config.transform.as_ref(),
                )),
            ),
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
