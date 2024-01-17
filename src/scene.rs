use super::camera::{Camera, CameraConfig};
use super::math::Ray;
use super::object::{HitRecord, Object, ObjectConfig};
use serde::Deserialize;
use std::sync::Arc;

pub struct Scene {
    pub camera: Arc<dyn Camera>,
    pub objects: Vec<Object>,
}

#[derive(Deserialize)]
pub struct SceneConfig {
    camera: CameraConfig,
    objects: Vec<ObjectConfig>,
}

impl Scene {
    pub fn from_config(config: &SceneConfig) -> Scene {
        let camera = config.camera.to_camera();

        let mut objects = Vec::new();

        for object_config in &config.objects {
            objects.push(object_config.to_object());
        }

        Scene {
            camera: camera,
            objects: objects,
        }
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
