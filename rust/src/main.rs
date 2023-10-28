use std::rc::Rc;

use rust_simple_raytracer::{
    materials::Dielectric, Camera, CameraParams, Hittables, Lambertain, Materials, Metal, Sphere,
    Vec3,
};

fn main() {
    let camera_params = CameraParams {
        // samples_per_pixel: 1,
        samples_per_pixel: 100,
        max_depth: 50,
        image_width: 400,
        fov: 100_f64,
        // focus_angle: 3_f64,
        // focus_distance: 0.5,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    let material_ground = Rc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.8_f64, 0.8_f64, 0_f64),
    }));
    let material_center_blue = Rc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
    }));
    let material_center_green = Rc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.1_f64, 0.8_f64, 0.1_f64),
    }));
    let material_metal = Rc::new(Materials::Metal(Metal::new(
        Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
        0.1_f64,
    )));
    let material_glass = Rc::new(Materials::Dielectric(Dielectric {
        index_of_reflectance: 1.5,
    }));

    let world = vec![
        Hittables::Sphere(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Rc::clone(&material_metal),
        }),
        Hittables::Sphere(Sphere {
            center: Vec3::new_int(0, 0, -1),
            radius: 0.5,
            material: Rc::clone(&material_center_blue),
        }),
        Hittables::Sphere(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Rc::clone(&material_glass),
        }),
        Hittables::Sphere(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: -0.4,
            material: Rc::clone(&material_glass),
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
