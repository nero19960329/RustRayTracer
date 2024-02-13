use super::math::{Point3D, Vec3D};
use super::object::Object;
use super::shapes::Shape;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point3D,
    pub normal: Vec3D,

    pub shape: Option<&'a dyn Shape>,
    pub object: Option<&'a Object>,
}
