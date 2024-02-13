mod camera;
mod common;
mod material;
mod math;
mod object;
mod renderer;
mod sampler;
mod scene;
mod shapes;
mod tracers;

use clap::Parser;
use log::info;
use renderer::{render, RenderConfig};
use scene::{Scene, SceneConfig};
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
    scene_config: String,

    #[arg(short, long)]
    render_config: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    env_logger::init();

    info!("RustRayTracer started.");
    let args = Args::parse();

    let render_config: RenderConfig =
        toml::from_str(&fs::read_to_string(args.render_config).unwrap())
            .expect("Failed to parse render config file");
    let scene_config: SceneConfig = toml::from_str(&fs::read_to_string(args.scene_config).unwrap())
        .expect("Failed to parse scene config file");
    let scene = Scene::from_config(&scene_config);
    let img = render(&render_config, &scene);
    img.save(&args.output).unwrap();
    info!("Image saved to {}.", args.output);
}
