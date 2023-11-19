use std::sync::Arc;

use rust_simple_raytracer::{
    Camera, CameraParams, Hittables, HittablesList, Lambertain, Materials, Quad, ScatterMaterials,
    Sphere, Vec3, BVH,
};

fn test_scene() {
    let material_ground = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Lambertain(
        Lambertain {
            albedo: Vec3::new(0.8_f64, 0.8_f64, 0_f64),
        },
    )));
    let left_red = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Lambertain(
        Lambertain {
            albedo: Vec3::new(1.0, 0.2, 0.2),
        },
    )));
    let light = Arc::new(Materials::LightMaterial);

    let mut hittable_list = HittablesList::new();
    hittable_list.add(Hittables::Sphere(Sphere::new(
        Vec3::new(0_f64, 1.0, -1_f64),
        0.5,
        light.clone(),
    )));
    hittable_list.add(Hittables::Quad(Quad::new(
        Vec3::new(-0.9, -0.5, -0.7),
        Vec3::new(0.2, 0.0, -0.7),
        Vec3::new_int(0, 1, 0),
        light.clone(),
    )));
    hittable_list.add(Hittables::Sphere(Sphere::new(
        Vec3::new(0_f64, 0.0, -1_f64),
        0.5,
        left_red,
    )));
    hittable_list.add(Hittables::Sphere(Sphere::new(
        Vec3::new(0_f64, -1000.5_f64, -1_f64),
        1000_f64,
        material_ground,
    )));
    let world = BVH::from_hittable_list(&hittable_list);

    let camera_params = CameraParams {
        samples_per_pixel: 50,
        max_depth: 50,
        fov: 80_f64,
        focus_angle: 0_f64,
        look_from: Vec3::new(0.25, 0.5, 0.0),
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
