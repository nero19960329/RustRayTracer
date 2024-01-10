use super::material::Material;
use super::math::{Point, Ray, Vec3};
use super::object::{HitRecord, Object, Plane, Sphere};

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut objects = Vec::new();

        objects.push(Object::Sphere(Sphere {    // light
            center: Point::new(50.0, 681.6 - 0.27, 81.6),
            radius: 600.0,
            material: Material::Emissive(Vec3::new(12.0, 12.0, 12.0)),
        }));

        objects.push(Object::Plane(Plane {  // left wall
            point: Point::new(1.0, 40.8, 81.6),
            normal: Vec3::new(1.0, 0.0, 0.0),
            material: Material::Lambertian(Vec3::new(0.75, 0.25, 0.25)),
        }));

        objects.push(Object::Plane(Plane {  // right wall
            point: Point::new(99.0, 40.8, 81.6),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            material: Material::Lambertian(Vec3::new(0.25, 0.25, 0.75)),
        }));
        
        objects.push(Object::Plane(Plane {  // back wall
            point: Point::new(50.0, 40.8, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: Material::Lambertian(Vec3::new(0.75, 0.75, 0.75)),
        }));

        objects.push(Object::Plane(Plane {  // front wall
            point: Point::new(50.0, 40.8, 300.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
            material: Material::Lambertian(Vec3::new(0.75, 0.75, 0.75)),
        }));

        objects.push(Object::Plane(Plane {  // bottom wall
            point: Point::new(50.0, 0.0, 81.6),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: Material::Lambertian(Vec3::new(0.75, 0.75, 0.75)),
        }));

        objects.push(Object::Plane(Plane {  // top wall
            point: Point::new(50.0, 81.6, 81.6),
            normal: Vec3::new(0.0, -1.0, 0.0),
            material: Material::Lambertian(Vec3::new(0.75, 0.75, 0.75)),
        }));

        objects.push(Object::Sphere(Sphere {
            center: Point::new(27.0, 16.5, 47.0),
            radius: 16.5,
            material: Material::Lambertian(Vec3::new(0.8, 0.6, 0.4)),
        }));

        objects.push(Object::Sphere(Sphere {
            center: Point::new(73.0, 16.5, 78.0),
            radius: 16.5,
            material: Material::Lambertian(Vec3::new(0.4, 0.6, 0.8)),
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
