use super::material::{Material, MaterialConfig};
use super::math::Ray;
use super::math::{Point3D, Vec3D};
use super::shapes::{Shape, ShapeConfig};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3D,
    pub normal: Vec3D,
    pub material: Option<Arc<dyn Material>>,
}

pub struct Object {
    pub shape: Shape,
    pub material: Arc<dyn Material>,
}

#[derive(Deserialize)]
pub struct ObjectConfig {
    pub shape: ShapeConfig,
    pub material: MaterialConfig,
}

impl ObjectConfig {
    pub fn to_object(&self) -> Object {
        Object {
            shape: self.shape.to_shape(),
            material: self.material.to_material(),
        }
    }
}

impl Object {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let hit_record = self.shape.intersect(ray, t_min, t_max);
        if hit_record.is_none() {
            return None;
        }
        let hit_record = hit_record.unwrap();
        let material = self.material.clone();
        Some(HitRecord {
            t: hit_record.t,
            p: hit_record.p,
            normal: hit_record.normal,
            material: Some(material),
        })
    }
}
