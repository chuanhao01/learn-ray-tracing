use rust_simple_raytracer::{Camera, CameraParams, Vec3};

fn main() {
    let camera_params = CameraParams {
        samples_per_pixel: 1,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);
    camera.render();
}
