extern crate cgmath;
extern crate image;
extern crate rand;

use cgmath::{ElementWise, InnerSpace};
use image::{ImageBuffer, RgbImage};
use rand::Rng;
use std::f32::consts::PI;

type Vec3 = cgmath::Vector3<f32>;
type Point = cgmath::Point3<f32>;

const MAX_DEPTH: u32 = 5;
const IMAGE_WIDTH: u32 = 320;
const IMAGE_HEIGHT: u32 = 240;
const SAMPLES_PER_PIXEL: u32 = 16;

#[derive(Clone)]
enum Material {
    Lambertian(Vec3),    // diffuse material, Vec3 stands for color
    Metallic(Vec3, f32), // metallic material, Vec3 stands for color, f32 stands for fuzziness
    Dielectric(f32),     // dielectric material, f32 stands for refraction index
    Emissive(Vec3),      // emissive material, Vec3 stands for color
}

struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}

struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn new(look_from: Point, look_at: Point, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    fn create_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin)
                .normalize(),
        }
    }
}

struct Sphere {
    center: Point,
    radius: f32,
    material: Material,
}

struct Plane {
    point: Point, // a point on the plane
    normal: Vec3, // normal vector of the plane
    material: Material,
}

enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray, t_min, t_max),
            Object::Plane(ref plane) => plane.intersect(ray, t_min, t_max),
        }
    }
}

struct Scene {
    objects: Vec<Object>,
}

struct HitRecord {
    t: f32,
    p: Point,
    normal: Vec3,
    material: Material,
}

impl Scene {
    fn new() -> Scene {
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

    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
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

impl Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = oc.dot(ray.direction);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord {
            t: root,
            p: point,
            normal,
            material: self.material.clone(),
        })
    }
}

impl Plane {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a: f32 = rng.gen_range(0.0..2.0 * PI);
    let z: f32 = rng.gen_range(0.0..1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth >= MAX_DEPTH {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = scene.intersect(ray) {
        match hit.material {
            Material::Emissive(color) => color,
            Material::Lambertian(color) => {
                let target = hit.p + hit.normal + random_in_unit_sphere();
                let new_ray = Ray {
                    origin: hit.p,
                    direction: target - hit.p,
                };
                color.mul_element_wise(trace(&new_ray, scene, depth + 1))
            }
            Material::Metallic(color, fuzz) => {
                // ... MCPT for metallic material
                // need mathematics equation instruction
                // raise exception for now
                panic!("Metallic material is not implemented yet!");
            }
            Material::Dielectric(refract_index) => {
                // ... MCPT for dielectric material
                // need mathematics equation instruction
                // raise exception for now
                panic!("Dielectric material is not implemented yet!");
            }
        }
    } else {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

fn reinhard_tone_mapping(color: Vec3) -> Vec3 {
    color.div_element_wise(color + Vec3::new(1.0, 1.0, 1.0))
}

fn gamma_correction(color: Vec3) -> Vec3 {
    color.map(|c| c.powf(1.0 / 2.2))
}

fn white_balance(color: Vec3, balance: Vec3) -> Vec3 {
    color.mul_element_wise(balance)
}

fn render(scene: &Scene, camera: &Camera) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let u_offset: f32 = rng.gen();
            let v_offset: f32 = rng.gen();
            let u = (x as f32 + u_offset) / (IMAGE_WIDTH - 1) as f32;
            let v = 1.0 - (y as f32 + v_offset) / (IMAGE_HEIGHT - 1) as f32;
            let ray = camera.create_ray(u, v);
            color += trace(&ray, scene, 0);
        }
        color /= SAMPLES_PER_PIXEL as f32;
        color = reinhard_tone_mapping(color);
        color = white_balance(color, Vec3::new(1.0, 1.0, 1.0));
        color = gamma_correction(color);
        *pixel = image::Rgb([
            (color.x * 255.0).min(255.0) as u8,
            (color.y * 255.0).min(255.0) as u8,
            (color.z * 255.0).min(255.0) as u8,
        ]);
    }

    img
}

fn main() {
    let scene = Scene::new();
    let camera = Camera::new(
        Point::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );
    let img = render(&scene, &camera);
    img.save("output.png").unwrap();
}
