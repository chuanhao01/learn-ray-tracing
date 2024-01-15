use std::sync::Arc;

use clap::Parser;
use image::RgbImage;
use rust_simple_raytracer::{
    Camera, CameraParams, Cli, Dielectric, HittableWithBBox, Lambertain, Materials, Metal,
    SolidColor, SpatialCheckeredTexture, Sphere, Vec3, BVH,
};

#[allow(clippy::vec_init_then_push)]
fn scene() -> RgbImage {
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
    let material_purple = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.8_f64, 0.0_f64, 0.8_f64),
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
        index_of_reflectance: 1.5,
    });

    let mut hittable_list: Vec<Arc<dyn HittableWithBBox>> = Vec::new();
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(-1.5, 1.0, -1.0),
        0.5,
        Materials::ScatterMaterial(material_red.clone()),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(-0.5, 1.0, -1.0),
        0.5,
        Materials::ScatterMaterial(material_green.clone()),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(0.5, 1.0, -1.0),
        0.5,
        Materials::ScatterMaterial(material_blue.clone()),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(1.5, 1.0, -1.0),
        0.5,
        Materials::ScatterMaterial(material_purple.clone()),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(1.5, 0.0, -1.0),
        0.5,
        Materials::ScatterMaterial(material_metal_fuzzy.clone()),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(0.5, 0.0, -1.0),
        0.5,
        Materials::ScatterMaterial(material_metal.clone()),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(-0.5, 0.0, -1.0),
        0.5,
        Materials::ScatterMaterial(material_glass.clone()),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(-0.5, 0.0, -1.0),
        -0.4,
        Materials::ScatterMaterial(material_glass.clone()),
    )));

    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new(0_f64, -100.5_f64, -1_f64),
        100_f64,
        Materials::ScatterMaterial(Arc::new(Lambertain {
            albedo: Arc::new(SpatialCheckeredTexture::from_colors(
                1.0,
                Vec3::new(0.2, 0.3, 0.1),
                Vec3::new(0.9, 0.9, 0.9),
            )),
        })),
    )));

    let world = BVH::from_hittables_list(hittable_list);

    let camera_params = CameraParams {
        samples_per_pixel: 50,
        max_depth: 30,
        image_width: 400,
        fov: 50_f64,
        focus_angle: 0_f64,
        look_from: Vec3::new_int(-1, 2, 1),
        look_at: Vec3::new_int(0, 0, -1),
        // focus_angle: 3_f64,
        // focus_distance: 0.4,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    camera.render_rgbimage(&world)
}

fn main() {
    let cli = Cli::parse();
    cli.save_image(scene());
}
