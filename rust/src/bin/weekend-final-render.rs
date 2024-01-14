use std::sync::Arc;

use rand::{thread_rng, Rng};
use rust_simple_raytracer::{
    materials::Dielectric, Camera, CameraParams, HittableWithBBox, Lambertain, Materials, Metal,
    SolidColor, Sphere, Vec3, BVH,
};

fn main() {
    let mut rng = thread_rng();

    let mut world: Vec<Arc<dyn HittableWithBBox>> = Vec::new();
    let ground_material = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.5, 0.5, 0.5),
        }),
    });
    world.push(Arc::new(Sphere::new(
        Vec3::new_int(0, -1000, 0),
        1000.0,
        Materials::ScatterMaterial(ground_material.clone()),
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
                    let albedo = Arc::new(SolidColor {
                        color: Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0),
                    });
                    Materials::ScatterMaterial(Arc::new(Lambertain {
                        albedo: albedo.clone(),
                    }))
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Arc::new(SolidColor {
                        color: Vec3::random(0.5, 1.0),
                    });
                    let fuzz = rng.gen_range(0.0..0.5);
                    Materials::ScatterMaterial(Arc::new(Metal::new(albedo.clone(), fuzz)))
                } else {
                    Materials::ScatterMaterial(Arc::new(Dielectric {
                        index_of_reflectance: 1.5,
                    }))
                };

                world.push(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
            }
        }
    }

    world.push(Arc::new(Sphere::new(
        Vec3::new_int(0, 1, 0),
        1.0,
        Materials::ScatterMaterial(Arc::new(Dielectric {
            index_of_reflectance: 1.5,
        })),
    )));
    world.push(Arc::new(Sphere::new(
        Vec3::new_int(-4, 1, 0),
        1.0,
        Materials::ScatterMaterial(Arc::new(Lambertain {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.4, 0.2, 0.1),
            }),
        })),
    )));
    world.push(Arc::new(Sphere::new(
        Vec3::new_int(4, 1, 0),
        1.0,
        Materials::ScatterMaterial(Arc::new(Metal::new(
            Arc::new(SolidColor {
                color: Vec3::new(0.7, 0.6, 0.5),
            }),
            0.0,
        ))),
    )));
    let world = BVH::from_hittables_list(world);

    let camera_params = CameraParams {
        samples_per_pixel: 100,
        max_depth: 50,
        image_width: 1200,
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
