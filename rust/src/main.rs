use std::rc::Rc;

use rust_simple_raytracer::{Camera, CameraParams, Hittables, Lambertain, Materials, Sphere, Vec3};

fn main() {
    let camera_params = CameraParams {
        // samples_per_pixel: 1,
        samples_per_pixel: 500,
        max_depth: 500,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    let material_ground = Rc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.8_f64, 0.8_f64, 0_f64),
    }));
    let material_center = Rc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
    }));

    let world = vec![
        Hittables::Sphere(Sphere {
            center: Vec3::new_int(0, 0, -1),
            radius: 0.5,
            material: material_center,
        }),
        Hittables::Sphere(Sphere {
            center: Vec3::new(0_f64, -100.5_f64, -1_f64),
            radius: 100_f64,
            material: material_ground,
        }),
    ];

    eprintln!("{:?}", camera);
    camera.render(&world);
}
