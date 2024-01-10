use super::material::Material;
use super::math::{Point, Ray, Vec3};
use cgmath::InnerSpace;
use log::info;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}

pub struct Plane {
    pub point: Point,
    pub normal: Vec3,
    pub material: Material,
}

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

pub struct HitRecord {
    pub t: f32,
    pub p: Point,
    pub normal: Vec3,
    pub material: Material,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = oc.dot(ray.direction);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // info!("oc: {:?}, a: {}, half_b: {}, c: {}, discriminant: {}", oc, a, half_b, c, discriminant);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        // info!("root: {}, t_min: {}, t_max: {}", root, t_min, t_max);
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let mut normal = (point - self.center).normalize();
        // XXX: if hit from inside, flip the normal
        if ray.direction.dot(normal) > 0.0 {
            normal = -normal;
        }

        Some(HitRecord {
            t: root,
            p: point,
            normal,
            material: self.material.clone(),
        })
    }
}

impl Plane {
    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denominator = self.normal.dot(ray.direction);
        if denominator.abs() < 1e-6 {
            return None;
        }

        let v = self.point - ray.origin;
        let distance = v.dot(self.normal) / denominator;
        if distance < t_min || distance > t_max {
            return None;
        }

        Some(HitRecord {
            t: distance,
            p: ray.at(distance),
            normal: self.normal,
            material: self.material.clone(),
        })
    }
}

impl Object {
    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray, t_min, t_max),
            Object::Plane(ref plane) => plane.intersect(ray, t_min, t_max),
        }
    }
}
