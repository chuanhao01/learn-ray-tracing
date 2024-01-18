use rand::{thread_rng, Rng};
use std::sync::Arc;

use rust_simple_raytracer::{
    construct_planar_quad_box, Camera, CameraParams, Dielectric, Diffuse, HittableWithBBox, Image,
    Lambertain, Materials, Metal, Quad, Rotation, SolidColor, Sphere, Translation, Vec3, Vec3Axis,
    BVH,
};

fn main() {
    let mut rng = thread_rng();
    let mut hittable_list: Vec<Arc<dyn HittableWithBBox>> = Vec::new();

    let ground_material = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.48, 0.83, 0.53),
        }),
    });
    let mut ground_boxes: Vec<Arc<dyn HittableWithBBox>> = Vec::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100_f64;
            let x0 = -1000_f64 + i as f64 * w;
            let z0 = -1000_f64 + j as f64 * w;
            let y0 = 0_f64;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1_f64..101_f64);
            let z1 = z0 + w;
            ground_boxes.push(Arc::new(construct_planar_quad_box(
                &Vec3::new(x0, y0, z0),
                &Vec3::new(x1, y1, z1),
                Materials::ScatterMaterial(ground_material.clone()),
            )));
        }
    }
    hittable_list.append(&mut ground_boxes);

    let diffuse_light = Arc::new(Diffuse { power: 7.0 });
    let light = Arc::new(Quad::new(
        Vec3::new_int(123, 554, 147),
        Vec3::new_int(300, 0, 0),
        Vec3::new_int(0, 0, 265),
        Materials::LightMaterial(diffuse_light.clone()),
    ));
    hittable_list.push(light);

    // let center1 = Vec3::new_int(400, 400, 200);
    // let center2 = center1 + Vec3::new_int(30, 0, 0);
    // let sphere_texture = Arc::new(Lambertain {
    //     albedo: Arc::new(SolidColor {
    //         color: Vec3::new(0.7, 0.3, 0.1),
    //     }),
    // });
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new_int(260, 150, 45),
        50.0,
        Materials::ScatterMaterial(Arc::new(Dielectric {
            index_of_reflectance: 1.5,
        })),
    )));
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new_int(0, 150, 145),
        50.0,
        Materials::ScatterMaterial(Arc::new(Metal::new(
            Arc::new(SolidColor {
                color: Vec3::new(0.8, 0.8, 0.9),
            }),
            1.0,
        ))),
    )));
    let earth_material = Arc::new(Lambertain {
        albedo: Arc::new(Image::new_with_color(
            1.0,
            "assets/earthmap.jpg",
            Vec3::new(0.0, 1.0, 1.0),
        )),
    });
    hittable_list.push(Arc::new(Sphere::new(
        Vec3::new_int(400, 200, 400),
        100.0,
        Materials::ScatterMaterial(earth_material.clone()),
    )));

    let mut box_of_spheres: Vec<Arc<dyn HittableWithBBox>> = Vec::new();
    let white = Arc::new(Lambertain {
        albedo: Arc::new(SolidColor {
            color: Vec3::new(0.73, 0.73, 0.73),
        }),
    });
    let ns = 1000;
    for _ in 0..ns {
        box_of_spheres.push(Arc::new(Sphere::new(
            Vec3::random(0.0, 165.0),
            10.0,
            Materials::ScatterMaterial(white.clone()),
        )));
    }
    hittable_list.push(Arc::new(Translation::new(
        Arc::new(Rotation::new(
            Arc::new(BVH::from_hittables_list(box_of_spheres)),
            Vec3Axis::Y,
            15.0,
        )),
        Vec3::new_int(-100, 270, 295),
    )));

    let world = BVH::from_hittables_list(hittable_list);

    let camera_params = CameraParams {
        aspect_ratio: 1.0,
        samples_per_pixel: 500,
        max_depth: 40,
        image_width: 800,
        fov: 40_f64,
        focus_angle: 0_f64,
        look_from: Vec3::new_int(478, 278, -600),
        look_at: Vec3::new_int(278, 278, 0),
        background: Vec3::new_int(0, 0, 0),
        focus_distance: 10.0,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    camera.render(&world);
}
