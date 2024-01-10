mod camera;
mod material;
mod math;
mod object;
mod raytracer;
mod renderer;
mod scene;

use camera::PerspectiveCamera;
use camera::SphericalPanoramicCamera;
use math::{Point, Vec3};
use renderer::{render, IMAGE_HEIGHT, IMAGE_WIDTH};
use scene::Scene;

fn main() {
    let scene = Scene::new();
    let camera = PerspectiveCamera::new(
        Point::new(50.0, 52.0, 295.6),
        Point::new(50.0, 39.4, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );
    // let camera = SphericalPanoramicCamera::new(
    //     Point::new(0.0, 0.0, 1.0)
    // );
    let img = render(&scene, &camera);
    img.save("output.png").unwrap();
}
