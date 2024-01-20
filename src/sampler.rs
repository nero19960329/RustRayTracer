use super::math::Point2U;
use rand::prelude::SliceRandom;
use rand::Rng;
use serde::Deserialize;

pub trait Sampler {
    fn start_pixel(&mut self, p: Point2U);
    fn get_1d(&mut self) -> f64;
    fn get_2d(&mut self) -> (f64, f64);
    fn start_next_sample(&mut self) -> bool;
    fn samples_per_pixel(&self) -> usize;
}

pub struct RandomSampler {
    rng: rand::rngs::ThreadRng,
    samples_per_pixel: usize,
    current_sample: usize,
}

#[derive(Deserialize)]
pub struct RandomSamplerConfig {
    pub samples_per_pixel: usize,
}

impl RandomSampler {
    pub fn new(samples_per_pixel: usize) -> Self {
        Self {
            rng: rand::thread_rng(),
            samples_per_pixel,
            current_sample: 0,
        }
    }
}

impl Sampler for RandomSampler {
    fn start_pixel(&mut self, _: Point2U) {
        self.current_sample = 0;
    }

    fn get_1d(&mut self) -> f64 {
        self.rng.gen()
    }

    fn get_2d(&mut self) -> (f64, f64) {
        (self.rng.gen(), self.rng.gen())
    }

    fn start_next_sample(&mut self) -> bool {
        if self.current_sample < self.samples_per_pixel {
            self.current_sample += 1;
            true
        } else {
            false
        }
    }

    fn samples_per_pixel(&self) -> usize {
        self.samples_per_pixel
    }
}

pub struct StratifiedSampler {
    samples_per_pixel: usize,
    x_strata: usize,
    y_strata: usize,
    samples_1d: Vec<Vec<f64>>,
    samples_2d: Vec<Vec<(f64, f64)>>,
    current_sample_index: usize,
    current_dimension: usize,
    rng: rand::rngs::ThreadRng,
}

#[derive(Deserialize)]
pub struct StratifiedSamplerConfig {
    pub samples_per_pixel: usize,
    pub x_strata: usize,
    pub y_strata: usize,
}

impl StratifiedSampler {
    pub fn new(
        samples_per_pixel: usize,
        x_strata: usize,
        y_strata: usize,
        dimensions: usize,
    ) -> Self {
        assert_eq!(
            x_strata * y_strata,
            samples_per_pixel,
            "x_strata * y_strata != samples_per_pixel"
        );

        Self {
            samples_per_pixel,
            x_strata,
            y_strata,
            samples_1d: vec![vec![0.0; samples_per_pixel]; dimensions],
            samples_2d: vec![vec![(0.0, 0.0); samples_per_pixel]; dimensions],
            current_sample_index: 0,
            current_dimension: 0,
            rng: rand::thread_rng(),
        }
    }

    fn generate_samples(&mut self) {
        for dim_samples in self.samples_1d.iter_mut() {
            for i in 0..self.samples_per_pixel {
                dim_samples[i] = (i as f64 + self.rng.gen::<f64>()) / self.samples_per_pixel as f64;
            }
            dim_samples.shuffle(&mut self.rng);
        }

        for dim_samples in self.samples_2d.iter_mut() {
            for i in 0..self.samples_per_pixel {
                let x_cell = i % self.x_strata;
                let y_cell = i / self.x_strata;

                let x = (x_cell as f64 + self.rng.gen::<f64>()) / self.x_strata as f64;
                let y = (y_cell as f64 + self.rng.gen::<f64>()) / self.y_strata as f64;

                dim_samples[i] = (x, y);
            }
            dim_samples.shuffle(&mut self.rng);
        }
    }
}

impl Sampler for StratifiedSampler {
    fn start_pixel(&mut self, _: Point2U) {
        self.current_sample_index = 0;
        self.current_dimension = 0;
        self.generate_samples();
    }

    fn get_1d(&mut self) -> f64 {
        if self.current_dimension >= self.samples_1d.len() {
            return self.rng.gen();
        }
        let sample = self.samples_1d[self.current_dimension][self.current_sample_index];
        self.current_dimension += 1;
        sample
    }

    fn get_2d(&mut self) -> (f64, f64) {
        if self.current_dimension >= self.samples_2d.len() {
            return (self.rng.gen(), self.rng.gen());
        }
        let sample = self.samples_2d[self.current_dimension][self.current_sample_index];
        self.current_dimension += 1;
        sample
    }

    fn start_next_sample(&mut self) -> bool {
        if self.current_sample_index < self.samples_per_pixel - 1 {
            self.current_sample_index += 1;
            self.current_dimension = 0;
            true
        } else {
            false
        }
    }

    fn samples_per_pixel(&self) -> usize {
        self.samples_per_pixel
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SamplerConfig {
    Random(RandomSamplerConfig),
    Stratified(StratifiedSamplerConfig),
}

impl SamplerConfig {
    pub fn to_sampler(&self) -> Box<dyn Sampler> {
        match self {
            SamplerConfig::Random(config) => Box::new(RandomSampler::new(config.samples_per_pixel)),
            SamplerConfig::Stratified(config) => Box::new(StratifiedSampler::new(
                config.samples_per_pixel,
                config.x_strata,
                config.y_strata,
                4,
            )),
        }
    }
}
