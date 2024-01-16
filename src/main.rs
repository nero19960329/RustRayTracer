mod camera;
mod material;
mod math;
mod object;
mod raytracer;
mod renderer;
mod scene;

use camera::PerspectiveCamera;
use clap::Parser;
use math::{Point, Vec3};
use renderer::{render, RenderConfig};
use scene::Scene;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "A simple raytracer written in Rust."
)]
struct Args {
    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let render_config: RenderConfig = toml::from_str(&fs::read_to_string(args.config).unwrap())
        .expect("Failed to parse config file");
    let scene = Scene::new();
    let camera = PerspectiveCamera::new(
        Point::new(50.0, 52.0, 295.6),
        Point::new(50.0, 39.4, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        render_config.image.width as f32 / render_config.image.height as f32,
    );
    let img = render(&render_config, &scene, &camera);
    img.save(args.output).unwrap();
}
