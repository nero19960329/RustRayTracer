use super::math::{Point, Ray, Vec3};
use cgmath::InnerSpace;
use std::f32::consts::PI;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
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

    pub fn create_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin)
                .normalize(),
        }
    }
}
