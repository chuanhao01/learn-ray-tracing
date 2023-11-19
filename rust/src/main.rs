use std::sync::Arc;

use rand::{thread_rng, Rng};
use rust_simple_raytracer::{
    Camera, CameraParams, Dielectric, Hittables, HittablesList, Lambertain, Metal,
    ScatterMaterials, Sphere, Vec3, BVH,
};

fn main() {
    let mut rng = thread_rng();

    let mut world = HittablesList::new();
    let ground_material = Arc::new(ScatterMaterials::Lambertain(Lambertain {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    }));
    world.add(Hittables::Sphere(Sphere::new(
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
                    ScatterMaterials::Lambertain(Lambertain { albedo })
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    ScatterMaterials::Metal(Metal::new(albedo, fuzz))
                } else {
                    ScatterMaterials::Dielectric(Dielectric {
                        index_of_reflectance: 1.5,
                    })
                };

                world.add(Hittables::Sphere(Sphere::new(
                    center,
                    0.2,
                    Arc::new(sphere_material),
                )))
            }
        }
    }

    world.add(Hittables::Sphere(Sphere::new(
        Vec3::new_int(0, 1, 0),
        1.0,
        Arc::new(ScatterMaterials::Dielectric(Dielectric {
            index_of_reflectance: 1.5,
        })),
    )));
    world.add(Hittables::Sphere(Sphere::new(
        Vec3::new_int(-4, 1, 0),
        1.0,
        Arc::new(ScatterMaterials::Lambertain(Lambertain {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        })),
    )));
    world.add(Hittables::Sphere(Sphere::new(
        Vec3::new_int(4, 1, 0),
        1.0,
        Arc::new(ScatterMaterials::Metal(Metal::new(
            Vec3::new(0.7, 0.6, 0.5),
            0.0,
        ))),
    )));
    let world = BVH::from_hittable_list(&world);

    let camera_params = CameraParams {
        samples_per_pixel: 20,
        max_depth: 5,
        image_width: 600,
        fov: 20_f64,
        look_from: Vec3::new_int(13, 2, 3),
        look_at: Vec3::new_int(0, 0, 0),
        v_up: Vec3::new_int(0, 1, 0),
        focus_angle: 0_f64,
        focus_distance: 10.0,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    camera.render(&world);
}
