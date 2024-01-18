use std::sync::Arc;

use rust_simple_raytracer::{
    Camera, CameraParams, CheckeredTexture, Diffuse, HittablesList, Lambertain, Materials, Quad,
    SolidColor, Sphere, Vec3, BVH,
};

fn test_scene() {
    let checkered_red = Materials::ScatterMaterial(Arc::new(Lambertain {
        albedo: Arc::new(CheckeredTexture::from_colors(
            30.0,
            Vec3::new(0.8, 0.1, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        )),
    }));
    let checkered_ground = Materials::ScatterMaterial(Arc::new(Lambertain {
        albedo: Arc::new(CheckeredTexture::from_colors(
            20.0,
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        )),
    }));
    let light = Arc::new(Diffuse { power: 4.0 });

    let mut hittable_list = HittablesList::new();
    hittable_list.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.5, -1_f64),
        2.5,
        Materials::LightMaterial(light.clone()),
    )));
    hittable_list.add(Arc::new(Quad::new(
        Vec3::new(2.5, 1.5, -2.0),
        Vec3::new(0.0, 0.0, 2.0),
        Vec3::new(0.0, 2.0, 0.0),
        Materials::LightMaterial(light.clone()),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        Vec3::new(0_f64, 2.0, -1_f64),
        2.0,
        checkered_red.clone(),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        Vec3::new(0_f64, -1000.5_f64, -1_f64),
        1000_f64,
        checkered_ground.clone(),
    )));
    let world = BVH::from_hittables_list(hittable_list.v);

    let camera_params = CameraParams {
        samples_per_pixel: 200,
        max_depth: 30,
        fov: 80_f64,
        focus_angle: 0_f64,
        image_width: 600,
        look_from: Vec3::new(-1.0, 3.0, 6.0),
        look_at: Vec3::new(0.0, 2.0, 0.0),
        background: Vec3::new(0.0, 0.0, 0.0),
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    // eprintln!("{}", world);
    camera.render(&world);
}

fn main() {
    test_scene();
}
