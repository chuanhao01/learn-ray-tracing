use rust_simple_raytracer::{CameraParams, Vec3, Camera};

fn main() {
    let camera_params = CameraParams{ ..Default::default()};
    let camera = Camera::new(camera_params);
    camera.render();
}
