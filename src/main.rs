mod camera;
mod material;
mod math;
mod object;
mod raytracer;
mod renderer;
mod scene;

use camera::PerspectiveCamera;
use math::{Point, Vec3};
use renderer::{render, IMAGE_HEIGHT, IMAGE_WIDTH};
use scene::Scene;

fn main() {
    let scene = Scene::new();
    let camera = PerspectiveCamera::new(
        Point::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );
    let img = render(&scene, &camera);
    img.save("output.png").unwrap();
}
