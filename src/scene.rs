use super::material::Material;
use super::math::{Point, Ray, Vec3};
use super::object::{HitRecord, Object, Plane, Sphere};

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut objects = Vec::new();

        objects.push(Object::Plane(Plane {
            point: Point::new(0.0, 0.0, -1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: Material::Lambertian(Vec3::new(0.8, 0.8, 0.8)),
        }));

        objects.push(Object::Sphere(Sphere {
            center: Point::new(0.0, 0.0, -0.5),
            radius: 0.5,
            material: Material::Lambertian(Vec3::new(0.8, 0.6, 0.2)),
        }));

        objects.push(Object::Plane(Plane {
            point: Point::new(0.0, 0.995, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
            material: Material::Emissive(Vec3::new(2.0, 2.0, 2.0)),
        }));

        Scene { objects }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = f32::MAX;

        for object in &self.objects {
            if let Some(temp_rec) = object.intersect(&ray, 0.001, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }
}
