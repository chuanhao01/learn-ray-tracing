use std::sync::Arc;

use rand::{thread_rng, Rng};
use rust_simple_raytracer::{
    materials::Dielectric, Camera, CameraParams, Hittables, Lambertain, Materials, Metal, Sphere,
    Vec3,
};

fn main() {
    let mut rng = thread_rng();

    let mut world: Vec<Hittables> = Vec::new();
    let ground_material = Arc::new(Materials::Lambertain(Lambertain {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    }));
    world.push(Hittables::Sphere(Sphere::new(
        Vec3::new_int(0, -1000, 0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // Lambertain
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    Materials::Lambertain(Lambertain { albedo })
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Materials::Metal(Metal::new(albedo, fuzz))
                } else {
                    Materials::Dielectric(Dielectric {
                        index_of_reflectance: 1.5,
                    })
                };

                world.push(Hittables::Sphere(Sphere::new(
                    center,
                    0.2,
                    Arc::new(sphere_material),
                )))
            }
        }
    }

    world.push(Hittables::Sphere(Sphere::new(
        Vec3::new_int(0, 1, 0),
        1.0,
        Arc::new(Materials::Dielectric(Dielectric {
            index_of_reflectance: 1.5,
        })),
    )));
    world.push(Hittables::Sphere(Sphere::new(
        Vec3::new_int(-4, 1, 0),
        1.0,
        Arc::new(Materials::Lambertain(Lambertain {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        })),
    )));
    world.push(Hittables::Sphere(Sphere::new(
        Vec3::new_int(4, 1, 0),
        1.0,
        Arc::new(Materials::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))),
    )));

    let camera_params = CameraParams {
        samples_per_pixel: 100,
        max_depth: 50,
        image_width: 400,
        fov: 20_f64,
        look_from: Vec3::new_int(13, 2, 3),
        look_at: Vec3::new_int(0, 0, 0),
        v_up: Vec3::new_int(0, 1, 0),
        focus_angle: 0.6_f64,
        focus_distance: 10.0,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    camera.render(&world);
}
