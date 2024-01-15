use std::sync::Arc;

use clap::Parser;
use image::RgbImage;
use rust_simple_raytracer::{
    Camera, CameraParams, CheckeredTexture, Cli, HittableWithBBox, Lambertain, Materials, Sphere,
    Vec3, BVH,
};

fn scene() -> RgbImage {
    let checkered = Materials::ScatterMaterial(Arc::new(Lambertain {
        albedo: Arc::new(CheckeredTexture::from_colors(
            100.0,
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        )),
    }));
    let hittable_list: Vec<Arc<dyn HittableWithBBox>> = vec![
        Arc::new(Sphere::new(
            Vec3::new_int(0, -10, 0),
            10.0,
            checkered.clone(),
        )),
        Arc::new(Sphere::new(
            Vec3::new_int(0, 10, 0),
            10.0,
            checkered.clone(),
        )),
    ];

    let world = BVH::from_hittables_list(hittable_list);

    let camera_params = CameraParams {
        samples_per_pixel: 100,
        max_depth: 50,
        image_width: 600,
        fov: 20_f64,
        focus_angle: 0_f64,
        look_from: Vec3::new_int(13, 2, 3),
        look_at: Vec3::new_int(0, 0, 0),
        // focus_angle: 3_f64,
        focus_distance: 1.0,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    camera.render_rgbimage(&world)
}

fn main() {
    let cli = Cli::parse();
    cli.save_image(scene());
}
