use super::math::Point2U;
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

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SamplerConfig {
    Random(RandomSamplerConfig),
}

impl SamplerConfig {
    pub fn to_sampler(&self) -> Box<dyn Sampler> {
        match self {
            SamplerConfig::Random(config) => Box::new(RandomSampler::new(config.samples_per_pixel)),
        }
    }
}
