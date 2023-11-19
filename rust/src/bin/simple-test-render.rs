use std::sync::Arc;

use rust_simple_raytracer::{
    Camera, CameraParams, Dielectric, Hittables, HittablesList, Lambertain, Materials, Metal, Quad,
    Sphere, Vec3, BVH,
};

fn test_scene() {
    let material_ground = Arc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.8_f64, 0.8_f64, 0_f64),
    }));

    let material_red = Arc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.8_f64, 0.0_f64, 0.0_f64),
    }));
    let material_green = Arc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.0_f64, 0.8_f64, 0.0_f64),
    }));
    let material_blue = Arc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.0_f64, 0.0_f64, 0.8_f64),
    }));

    let material_metal = Arc::new(Materials::Metal(Metal::new(
        Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
        0.0_f64,
    )));
    let material_metal_fuzzy = Arc::new(Materials::Metal(Metal::new(
        Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
        0.3_f64,
    )));
    let material_glass = Arc::new(Materials::Dielectric(Dielectric {
        index_of_reflectance: 1.4,
    }));

    let mut hittable_list = HittablesList::new();
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(-1.0, 1.0, -0.7),
    //     0.5,
    //     Arc::clone(&material_red),
    // )));
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(0.0, 1.0, -0.7),
    //     0.5,
    //     Arc::clone(&material_green),
    // )));
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(1.0, 1.0, -0.7),
    //     0.5,
    //     Arc::clone(&material_blue),
    // )));
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(-0.8, 0.0, -1.0),
    //     0.2,
    //     Arc::clone(&material_metal),
    // )));
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(-0.3, 0.0, -1.0),
    //     0.2,
    //     Arc::clone(&material_metal_fuzzy),
    // )));
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(0.3, 0.0, -1.0),
    //     0.2,
    //     Arc::clone(&material_glass),
    // )));
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(0.8, 0.0, -1.0),
    //     -0.15,
    //     Arc::clone(&material_glass),
    // )));
    // hittable_list.add(Hittables::Sphere(Sphere::new(
    //     Vec3::new(0.8, 0.0, -1.0),
    //     0.2,
    //     Arc::clone(&material_glass),
    // )));

    hittable_list.add(Hittables::Sphere(Sphere::new(
        Vec3::new(-0.25, 0.0, -1.0),
        0.4,
        Arc::clone(&material_red),
    )));
    hittable_list.add(Hittables::Quad(Quad::new(
        Vec3::new(0.25, -0.25, -1.5),
        Vec3::new(0.75, 0.0, 0.5),
        Vec3::new(0.0, 0.75, 0.0),
        material_metal,
    )));
    hittable_list.add(Hittables::Sphere(Sphere::new(
        Vec3::new(0_f64, -100.5_f64, -1_f64),
        100_f64,
        material_ground,
    )));
    let world = BVH::from_hittable_list(&hittable_list);

    let camera_params = CameraParams {
        samples_per_pixel: 50,
        max_depth: 20,
        image_width: 400,
        fov: 70_f64,
        focus_angle: 0_f64,
        // focus_angle: 3_f64,
        // focus_distance: 0.4,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    eprintln!("{}", world);
    camera.render(&world);
    // eprintln!("len: {}", hittable_list.len());
}

fn main() {
    test_scene();
}
