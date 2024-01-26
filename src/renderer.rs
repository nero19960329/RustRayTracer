use super::math::{Point2U, Vec3D, Vec3DConfig};
use super::raytracer::trace;
use super::sampler::SamplerConfig;
use super::scene::Scene;
use cgmath::ElementWise;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct RenderConfig {
    pub image: ImageConfig,
    pub sampler: SamplerConfig,
    post_processing: PostProcessingConfig,
    performance: PerformanceConfig,
}

#[derive(Deserialize)]
pub struct ImageConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
struct PostProcessingConfig {
    tone_mapping: Option<String>,
    gamma_correction: bool,
    white_balance: Option<Vec3DConfig>,
}

#[derive(Deserialize)]
struct PerformanceConfig {
    parallelism: Option<usize>,
}

fn reinhard_tone_mapping(color: Vec3D) -> Vec3D {
    color.div_element_wise(color + Vec3D::new(1.0, 1.0, 1.0))
}

fn gamma_correction(color: Vec3D) -> Vec3D {
    color.map(|c| c.powf(1.0 / 2.2))
}

fn white_balance(color: Vec3D, balance: Vec3D) -> Vec3D {
    color.mul_element_wise(balance)
}

fn post_process(color: Vec3D, config: &PostProcessingConfig) -> Vec3D {
    let color = if let Some(tone_mapping) = &config.tone_mapping {
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

pub fn render(config: &RenderConfig, scene: &Scene) -> RgbImage {
    let parallelism = config.performance.parallelism.unwrap_or(1);
    rayon::ThreadPoolBuilder::new()
        .num_threads(parallelism)
        .build_global()
        .unwrap();

    let pixel_count = config.image.width as usize * config.image.height as usize;
    let progress_bar = Arc::new(ProgressBar::new(pixel_count as u64));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
            )
            .expect("Failed to set progress bar style")
            .progress_chars("#>-"),
    );
    let pb = progress_bar.clone();

    let tile_size = 16;
    let tiles_x = (config.image.width as usize + tile_size - 1) / tile_size;
    let tiles_y = (config.image.height as usize + tile_size - 1) / tile_size;
    let tile_count = tiles_x * tiles_y;
    let img = Arc::new(Mutex::new(ImageBuffer::new(
        config.image.width,
        config.image.height,
    )));

    (0..tile_count)
        .into_par_iter()
        .for_each_with(pb, |pb, tile_index| {
            let tile_x = tile_index % tiles_x;
            let tile_y = tile_index / tiles_x;
            let x_start = tile_x * tile_size;
            let y_start = tile_y * tile_size;
            let x_end = (x_start + tile_size).min(config.image.width as usize);
            let y_end = (y_start + tile_size).min(config.image.height as usize);

            let mut sampler = config.sampler.to_sampler();
            let spp = sampler.samples_per_pixel();
            for y in y_start..y_end {
                for x in x_start..x_end {
                    sampler.start_pixel(Point2U::new(x as u32, y as u32));
                    let mut color = Vec3D::new(0.0, 0.0, 0.0);
                    loop {
                        let (u_offset, v_offset) = sampler.get_2d();
                        let u = (x as f64 + u_offset + 0.5) / config.image.width as f64;
                        let v = 1.0 - (y as f64 + v_offset + 0.5) / config.image.height as f64;
                        let ray = scene.camera.create_ray(u, v);
                        color += trace(&ray, scene, 0, &mut *sampler);
                        if !sampler.start_next_sample() {
                            break;
                        }
                    }
                    color /= spp as f64;
                    color = post_process(color, &config.post_processing);
                    let mut img = img.lock().unwrap();
                    let img_pixel = img.get_pixel_mut(x as u32, y as u32);
                    *img_pixel = image::Rgb([
                        (color.x * 255.0).min(255.0) as u8,
                        (color.y * 255.0).min(255.0) as u8,
                        (color.z * 255.0).min(255.0) as u8,
                    ]);

                    pb.inc(1);
                }
            }
        });
    Arc::try_unwrap(progress_bar)
        .expect("Failed to unwrap progress bar")
        .finish_with_message("Render complete!");

    Arc::try_unwrap(img)
        .expect("Failed to unwrap image")
        .into_inner()
        .unwrap()
}
