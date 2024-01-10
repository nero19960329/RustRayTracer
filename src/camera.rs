use super::math::{Point, Ray, Vec3};
use cgmath::InnerSpace;
use std::f32::consts::PI;

pub trait Camera {
    fn create_ray(&self, s: f32, t: f32) -> Ray;
}

pub struct PerspectiveCamera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl PerspectiveCamera {
    pub fn new(look_from: Point, look_at: Point, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Self {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn create_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin)
                .normalize(),
        }
    }
}

pub struct OrthographicCamera {
    origin: Point,
    direction: Vec3,
    horizontal: Vec3,   // horizontal and vertical are the basis of the view plane
    vertical: Vec3,
}

impl OrthographicCamera {
    pub fn new(look_from: Point, look_at: Point, vup: Vec3, width: f32, height: f32) -> Self {
        let direction = (look_at - look_from).normalize();
        let horizontal = vup.cross(direction).normalize() * width;  // XXX: check left/right is reversed
        let vertical = direction.cross(horizontal).normalize() * height;
        
        Self {
            origin: look_from,
            direction,
            horizontal,
            vertical,
        }
    }
}

impl Camera for OrthographicCamera {
    fn create_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin + (s - 0.5) * self.horizontal + (t - 0.5) * self.vertical,
            direction: self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{point_approx_eq, vec3_approx_eq};

    #[test]
    fn test_perspective_camera() {
        let camera = PerspectiveCamera::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            2.0,
        );
        let ray = camera.create_ray(0.5, 0.5);
        assert!(point_approx_eq(ray.origin, Point::new(0.0, 0.0, 0.0), 1e-6));
        assert!(vec3_approx_eq(
            ray.direction,
            Vec3::new(0.0, 0.0, -1.0),
            1e-6
        ));
    }

    #[test]
    fn test_orthographic_camera() {
        let camera = OrthographicCamera::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            2.0,
            2.0,
        );

        let mut ray = camera.create_ray(0.5, 0.5);
        assert!(point_approx_eq(ray.origin, Point::new(0.0, 0.0, 0.0), 1e-6));
        assert!(vec3_approx_eq(
            ray.direction,
            Vec3::new(0.0, 0.0, -1.0),
            1e-6
        ));

        ray = camera.create_ray(0.0, 0.0);
        assert!(point_approx_eq(ray.origin, Point::new(1.0, -1.0, 0.0), 1e-6));
        assert!(vec3_approx_eq(
            ray.direction,
            Vec3::new(0.0, 0.0, -1.0),
            1e-6
        ));
    }
}
