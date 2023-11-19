use std::sync::Arc;

use rust_simple_raytracer::{
    Camera, CameraParams, Dielectric, Hittables, Lambertain, Metal, ScatterMaterials, Sphere, Vec3,
};

#[allow(clippy::vec_init_then_push)]
fn test_scene() {
    let camera_params = CameraParams {
        samples_per_pixel: 1,
        max_depth: 2,
        image_width: 400,
        fov: 70_f64,
        // focus_angle: 3_f64,
        // focus_distance: 0.4,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    let material_ground = Arc::new(ScatterMaterials::Lambertain(Lambertain {
        albedo: Vec3::new(0.8_f64, 0.8_f64, 0_f64),
    }));

    let material_red = Arc::new(ScatterMaterials::Lambertain(Lambertain {
        albedo: Vec3::new(0.8_f64, 0.0_f64, 0.0_f64),
    }));
    let material_green = Arc::new(ScatterMaterials::Lambertain(Lambertain {
        albedo: Vec3::new(0.0_f64, 0.8_f64, 0.0_f64),
    }));
    let material_blue = Arc::new(ScatterMaterials::Lambertain(Lambertain {
        albedo: Vec3::new(0.0_f64, 0.0_f64, 0.8_f64),
    }));

    let material_metal = Arc::new(ScatterMaterials::Metal(Metal::new(
        Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
        0.0_f64,
    )));
    let material_metal_fuzzy = Arc::new(ScatterMaterials::Metal(Metal::new(
        Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
        0.3_f64,
    )));
    let material_glass = Arc::new(ScatterMaterials::Dielectric(Dielectric {
        index_of_reflectance: 1.4,
    }));

    let mut hittable_list: Vec<Hittables> = Vec::new();
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(-1.0, 1.0, -0.7),
        0.5,
        Arc::clone(&material_red),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(0.0, 1.0, -0.7),
        0.5,
        Arc::clone(&material_green),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(1.0, 1.0, -0.7),
        0.5,
        Arc::clone(&material_blue),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(-0.8, 0.0, -1.0),
        0.2,
        Arc::clone(&material_metal),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(-0.3, 0.0, -1.0),
        0.2,
        Arc::clone(&material_metal_fuzzy),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(0.3, 0.0, -1.0),
        0.2,
        Arc::clone(&material_glass),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(0.8, 0.0, -1.0),
        -0.15,
        Arc::clone(&material_glass),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(0.8, 0.0, -1.0),
        0.2,
        Arc::clone(&material_glass),
    )));
    hittable_list.push(Hittables::Sphere(Sphere::new(
        Vec3::new(0_f64, -100.5_f64, -1_f64),
        100_f64,
        material_ground,
    )));
    let world = hittable_list;

    eprintln!("{:?}", camera);
    camera.render(&world);
}

fn main() {
    test_scene();
}
