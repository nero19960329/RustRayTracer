use super::math::{Point3D, Point3DConfig, Ray, Vec3D, Vec3DConfig};
use cgmath::InnerSpace;
use serde::Deserialize;
use std::f64::consts::PI;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Camera: Sync + Send + Debug {
    fn create_ray(&self, s: f64, t: f64) -> Ray;
}

#[derive(Debug)]
pub struct PerspectiveCamera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vec3D,
    vertical: Vec3D,
}

impl PerspectiveCamera {
    pub fn new(look_from: Point3D, look_at: Point3D, vup: Vec3D, vfov: f64, aspect: f64) -> Self {
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
    fn create_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin)
                .normalize(),
        }
    }
}

#[derive(Deserialize)]
pub struct PerspectiveCameraConfig {
    look_from: Point3DConfig,
    look_at: Point3DConfig,
    vup: Vec3DConfig,
    vfov: f64,
    aspect: f64,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum CameraConfig {
    Perspective(PerspectiveCameraConfig),
}

impl CameraConfig {
    pub fn to_camera(&self) -> Arc<dyn Camera> {
        match self {
            CameraConfig::Perspective(config) => Arc::new(PerspectiveCamera::new(
                config.look_from.to_point(),
                config.look_at.to_point(),
                config.vup.to_vec3(),
                config.vfov,
                config.aspect,
            )),
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
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, -1.0),
            Vec3D::new(0.0, 1.0, 0.0),
            90.0,
            2.0,
        );
        let ray = camera.create_ray(0.5, 0.5);
        assert!(point_approx_eq(
            ray.origin,
            Point3D::new(0.0, 0.0, 0.0),
            1e-6
        ));
        assert!(vec3_approx_eq(
            ray.direction,
            Vec3D::new(0.0, 0.0, -1.0),
            1e-6
        ));
    }
}
