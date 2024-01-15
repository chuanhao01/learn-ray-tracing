use std::sync::Arc;

use clap::Parser;
use image::RgbImage;
use rust_simple_raytracer::{
    Camera, CameraParams, Cli, Dielectric, Disk, HittablesList, Lambertain, Materials, Metal, Quad,
    SolidColor, Sphere, Triangle, Vec3, BVH,
};

fn scene() -> RgbImage {
    let left_red = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(1.0, 0.2, 0.2),
        }),
    });
    let back_green = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.2, 1.0, 0.2),
        }),
    });
    let right_blue = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.2, 0.2, 1.0),
        }),
    });
    let upper_orange = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(1.0, 0.5, 0.0),
        }),
    });
    let lower_teal = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.2, 0.8, 0.8),
        }),
    });

    let material_metal = Arc::new(Metal::new(
        Arc::new(SolidColor {
            color: Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
        }),
        0.0_f64,
    ));
    let material_metal_fuzzy = Arc::new(Metal::new(
        Arc::new(SolidColor {
            color: Vec3::new(0.1_f64, 0.2_f64, 0.5_f64),
        }),
        0.3_f64,
    ));
    let material_glass = Arc::new(Dielectric {
        index_of_reflectance: 2.0,
    });

    let mut hittable_list = HittablesList::new();
    hittable_list.add(Arc::new(Disk::new(
        Vec3::new_int(-3, -2, 3),
        Vec3::new_int(0, 0, -4),
        Vec3::new_int(0, 4, 0),
        1.0,
        Materials::ScatterMaterial(left_red.clone()),
    )));
    hittable_list.add(Arc::new(Quad::new(
        Vec3::new_int(-2, -2, 0),
        Vec3::new_int(4, 0, 0),
        Vec3::new_int(0, 4, 0),
        Materials::ScatterMaterial(back_green.clone()),
    )));
    hittable_list.add(Arc::new(Triangle::new(
        Vec3::new_int(3, -2, 1),
        Vec3::new_int(0, 0, 4),
        Vec3::new_int(0, 4, 0),
        Materials::ScatterMaterial(material_metal.clone()),
    )));
    hittable_list.add(Arc::new(Disk::new(
        Vec3::new_int(-2, 3, 1),
        Vec3::new(4.0, 0.1, 0.0),
        Vec3::new_int(0, 0, 4),
        1.0,
        Materials::ScatterMaterial(upper_orange.clone()),
    )));
    hittable_list.add(Arc::new(Triangle::new(
        Vec3::new_int(-2, -3, 5),
        Vec3::new_int(4, 0, 0),
        Vec3::new_int(0, 0, -4),
        Materials::ScatterMaterial(lower_teal.clone()),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 2.0),
        0.8,
        Materials::ScatterMaterial(material_metal_fuzzy.clone()),
    )));
    let world = BVH::from_hittables_list(hittable_list.v);

    let camera_params = CameraParams {
        aspect_ratio: 1.0,
        samples_per_pixel: 200,
        max_depth: 50,
        image_width: 400,
        fov: 80_f64,
        focus_angle: 0_f64,
        look_from: Vec3::new(0.0, 0.0, 9.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        // focus_distance: 0.4,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    // eprintln!("{}", world);
    camera.render_rgbimage(&world)
}

fn main() {
    let cli = Cli::parse();
    cli.save_image(scene());
}
