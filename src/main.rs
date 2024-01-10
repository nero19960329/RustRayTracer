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
use fern::Dispatch;

fn main() {
    Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("{}[{}]: {}", record.target(), record.level(), message)))
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply().unwrap();

    let scene = Scene::new();
    let camera = PerspectiveCamera::new(
        Point::new(50.0, 52.0, 250.0),
        Point::new(50.0, 39.4, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );
    // let camera = SphericalPanoramicCamera::new(
    //     Point::new(50.0, 52.0, 250.0)
    // );
    let img = render(&scene, &camera);
    img.save("output.png").unwrap();
}
