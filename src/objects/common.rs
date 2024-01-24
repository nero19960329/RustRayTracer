use super::super::material::Material;
use super::super::math::{Point3D, Vec3D};
use std::sync::Arc;

#[derive(Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3D,
    pub normal: Vec3D,
    pub material: Arc<dyn Material>,
}
