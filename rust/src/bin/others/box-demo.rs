use std::sync::Arc;

use rust_simple_raytracer::{
    construct_planar_quad_box, Camera, CameraParams, Dielectric, Disk, Hittables, HittablesList,
    Lambertain, Materials, Metal, Quad, ScatterMaterials, Sphere, Triangle, Vec3, BVH,
};

fn test_scene() {
    let left_red = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Lambertain(
        Lambertain {
            albedo: Vec3::new(1.0, 0.2, 0.2),
        },
    )));
    let material_metal = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Metal(
        Metal::new(Vec3::new(0.1_f64, 0.2_f64, 0.5_f64), 0.2_f64),
    )));
    let material_glass = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Dielectric(
        Dielectric {
            index_of_reflectance: 1.4,
        },
    )));

    let mut hittable_list = HittablesList::new();
    hittable_list.append(&mut construct_planar_quad_box(
        &Vec3::new(-0.5, -0.5, -1.0),
        &Vec3::new(0.5, 0.5, -2.0),
        material_glass.clone(),
    ));
    hittable_list.append(&mut construct_planar_quad_box(
        &Vec3::new(1.0, -0.5, -1.5),
        &Vec3::new(1.5, 0.5, -2.5),
        left_red.clone(),
    ));
    let world = BVH::from_hittable_list(&hittable_list);

    let camera_params = CameraParams {
        aspect_ratio: 1.0,
        samples_per_pixel: 50,
        max_depth: 50,
        image_width: 600,
        fov: 80_f64,
        focus_angle: 0_f64,
        look_from: Vec3::new(2.0, 0.0, 0.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        // focus_distance: 0.4,
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
