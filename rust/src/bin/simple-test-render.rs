use std::sync::Arc;

use rust_simple_raytracer::{
    Camera, CameraParams, Dielectric, HittablesList, Lambertain, Materials, Metal, Quad,
    SolidColor, Sphere, Vec3, Vec3Axis, BVH,
};

fn test_scene() {
    let material_ground = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.8_f64, 0.8_f64, 0_f64),
        }),
    });

    let material_red = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.8_f64, 0.0_f64, 0.0_f64),
        }),
    });
    let material_green = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.0_f64, 0.8_f64, 0.0_f64),
        }),
    });
    let material_blue = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.0_f64, 0.0_f64, 0.8_f64),
        }),
    });

    let material_metal = Arc::new(Metal::new(Vec3::new(0.1_f64, 0.2_f64, 0.5_f64), 0.0_f64));
    let material_metal_fuzzy = Arc::new(Metal::new(Vec3::new(0.1_f64, 0.2_f64, 0.5_f64), 0.3_f64));
    let material_glass = Arc::new(Dielectric {
        index_of_reflectance: 1.4,
    });

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

    // let center_ball = Arc::new(Hittables::Sphere(Sphere::new(
    //     Vec3::new(-0.25, 0.0, -1.0),
    //     0.4,
    //     Arc::new(Materials::ScatterMaterial(material_red)),
    // )));
    // let center_box = construct_planar_quad_box(
    //     &Vec3::new(-0.4, -0.4, -1.0),
    //     &Vec3::new(0.4, 0.4, -1.4),
    //     Arc::new(Materials::ScatterMaterial(material_red)),
    // );
    // hittable_list.append(&mut center_box.clone());
    // hittable_list.append(
    //     &mut center_box
    //         .iter()
    //         .map(|plane| {
    //             Arc::new(Hittables::Translation(Translation::new(
    //                 plane.clone(),
    //                 Vec3::new(-1.0, 0.0, 0.0),
    //             )))
    //         })
    //         .collect::<Vec<_>>(),
    // );
    // hittable_list.add(Arc::new(Hittables::Quad(Quad::new(
    //     Vec3::new(0.25, -0.25, -1.5),
    //     Vec3::new(0.75, 0.0, 0.5),
    //     Vec3::new(0.0, 0.75, 0.0),
    //     Arc::new(Materials::ScatterMaterial(material_metal)),
    // ))));
    let center_quad = Arc::new(Quad::new(
        Vec3::new(-0.5, 0.0, -1.0),
        Vec3::new(1.0, 0.0, 0.0)
            .rotate_about_axis(&Vec3Axis::Y, 30.0)
            .rotate_about_axis(&Vec3Axis::X, 30.0),
        // Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.5, 0.0)
            .rotate_about_axis(&Vec3Axis::X, 30.0)
            .rotate_about_axis(&Vec3Axis::Y, 30.0),
        // Vec3::new(0.0, 0.5, 0.0),
        Materials::ScatterMaterial(material_blue),
    ));
    hittable_list.add(center_quad.clone());
    hittable_list.add(Arc::new(Sphere::new(
        Vec3::new(0_f64, -100.5_f64, -1_f64),
        100_f64,
        Materials::ScatterMaterial(material_green),
    )));

    // let world = Sphere::new(
    //     Vec3::new(0_f64, -100.5_f64, -1_f64),
    //     100_f64,
    //     Arc::new(Materials::ScatterMaterial(material_green)),
    // );
    // let world = hittable_list;
    let world = BVH::from_hittables_list(hittable_list.v);

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
    camera.render(&world);
}

fn main() {
    test_scene();
}
