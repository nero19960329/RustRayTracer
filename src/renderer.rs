use super::camera::Camera;
use super::math::Vec3;
use super::raytracer::trace;
use super::scene::Scene;
use cgmath::ElementWise;
use image::{ImageBuffer, RgbImage};
use rand::Rng;

pub const IMAGE_WIDTH: u32 = 320;
pub const IMAGE_HEIGHT: u32 = 240;
pub const SAMPLES_PER_PIXEL: u32 = 16;

fn reinhard_tone_mapping(color: Vec3) -> Vec3 {
    color.div_element_wise(color + Vec3::new(1.0, 1.0, 1.0))
}

fn gamma_correction(color: Vec3) -> Vec3 {
    color.map(|c| c.powf(1.0 / 2.2))
}

fn white_balance(color: Vec3, balance: Vec3) -> Vec3 {
    color.mul_element_wise(balance)
}

pub fn render<C: Camera>(scene: &Scene, camera: &C) -> RgbImage {
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
