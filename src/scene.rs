use super::material::Material;
use super::math::{Point, Ray, Vec3};
use super::object::{HitRecord, Object, Plane, Sphere};

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut objects = Vec::new();

        // objects.push(Object::Plane(Plane {
        //     point: Point::new(0.0, 0.0, -1.0),
        //     normal: Vec3::new(0.0, 0.0, 1.0),
        //     material: Material::Lambertian(Vec3::new(0.8, 0.8, 0.8)),
        // }));

        // objects.push(Object::Sphere(Sphere {
        //     center: Point::new(0.0, 0.0, -0.5),
        //     radius: 0.5,
        //     material: Material::Lambertian(Vec3::new(0.8, 0.6, 0.2)),
        // }));

        // objects.push(Object::Plane(Plane {
        //     point: Point::new(0.0, 0.995, 0.0),
        //     normal: Vec3::new(0.0, -1.0, 0.0),
        //     material: Material::Emissive(Vec3::new(2.0, 2.0, 2.0)),
        // }));

        objects.push(Object::Sphere(Sphere {    // left wall
            center: Point::new(1e5 + 1.0, 40.8, 81.6),
            radius: 1e5,
            material: Material::Lambertian(Vec3::new(0.75, 0.25, 0.25)),
        }));

        // objects.push(Object::Sphere(Sphere {    // right wall
        //     center: Point::new(-1e5 + 99.0, 40.8, 81.6),
        //     radius: 1e5,
        //     material: Material::Lambertian(Vec3::new(0.25, 0.25, 0.75)),
        // }));

        // objects.push(Object::Sphere(Sphere {    // back wall
        //     center: Point::new(50.0, 40.8, 1e5),
        //     radius: 1e5,
        //     material: Material::Lambertian(Vec3::new(0.75, 0.75, 0.75)),
        // }));

        // objects.push(Object::Sphere(Sphere {    // front wall
        //     center: Point::new(50.0, 40.8, -1e5 + 170.0),
        //     radius: 1e5,
        //     material: Material::Lambertian(Vec3::new(0.0, 0.0, 0.0)),
        // }));

        // objects.push(Object::Sphere(Sphere {    // bottom wall
        //     center: Point::new(50.0, 1e5, 81.6),
        //     radius: 1e5,
        //     material: Material::Lambertian(Vec3::new(0.75, 0.75, 0.75)),
        // }));

        objects.push(Object::Sphere(Sphere {    // light
            center: Point::new(50.0, 681.6 - 0.27, 81.6),
            radius: 600.0,
            material: Material::Emissive(Vec3::new(12.0, 12.0, 12.0)),
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
