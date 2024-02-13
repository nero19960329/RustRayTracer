use super::common::HitRecord;
use super::material::{Material, MaterialConfig};
use super::math::Ray;
use super::shapes::{Shape, ShapeConfig};
use serde::Deserialize;
use std::sync::Arc;

pub struct Object {
    pub shape: Arc<dyn Shape>,
    pub material: Arc<dyn Material>,
}

impl Object {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let hit_record = self.shape.intersect(ray, t_min, t_max);
        if hit_record.is_none() {
            return None;
        }

        let mut hit_record = hit_record.unwrap();
        hit_record.object = Some(self);
        Some(hit_record)
    }
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
