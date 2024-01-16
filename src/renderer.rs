use super::camera::Camera;
use super::math::Vec3;
use super::raytracer::trace;
use super::scene::Scene;
use cgmath::ElementWise;
use image::{ImageBuffer, RgbImage};
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::Deserialize;

#[derive(Deserialize)]
struct Vec3Config {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3Config {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(Deserialize)]
pub struct RenderConfig {
    pub image: ImageConfig,
    post_processing: PostProcessingConfig,
    performance: PerformanceConfig,
}

#[derive(Deserialize)]
pub struct ImageConfig {
    pub width: u32,
    pub height: u32,
    samples_per_pixel: u32,
}

#[derive(Deserialize)]
struct PostProcessingConfig {
    tone_mapping: Option<String>,
    gamma_correction: bool,
    white_balance: Option<Vec3Config>,
}

#[derive(Deserialize)]
struct PerformanceConfig {
    parallelism: Option<usize>,
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

fn post_process(color: Vec3, config: &PostProcessingConfig) -> Vec3 {
    if let Some(tone_mapping) = &config.tone_mapping {
        match tone_mapping.as_str() {
            "reinhard" => reinhard_tone_mapping(color),
            _ => color,
        }
    } else {
        color
    };
    let color = if config.gamma_correction {
        gamma_correction(color)
    } else {
        color
    };
    let color = if let Some(white_balance_config) = &config.white_balance {
        white_balance(color, white_balance_config.to_vec3())
    } else {
        color
    };
    color
}

pub fn render<C: Camera>(config: &RenderConfig, scene: &Scene, camera: &C) -> RgbImage {
    let parallelism = config.performance.parallelism.unwrap_or(1);
    rayon::ThreadPoolBuilder::new()
        .num_threads(parallelism)
        .build_global()
        .unwrap();

    let mut img: RgbImage = ImageBuffer::new(config.image.width, config.image.height);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut rng = rand::thread_rng();
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..config.image.samples_per_pixel {
                let u_offset: f32 = rng.gen();
                let v_offset: f32 = rng.gen();
                let u = (x as f32 + u_offset + 0.5) / config.image.width as f32;
                let v = 1.0 - (y as f32 + v_offset + 0.5) / config.image.height as f32;
                let ray = camera.create_ray(u, v);
                color += trace(&ray, scene, 0);
            }
            color /= config.image.samples_per_pixel as f32;
            color = post_process(color, &config.post_processing);
            *pixel = image::Rgb([
                (color.x * 255.0).min(255.0) as u8,
                (color.y * 255.0).min(255.0) as u8,
                (color.z * 255.0).min(255.0) as u8,
            ]);
        });

    img
}
