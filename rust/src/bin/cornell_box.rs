use std::sync::Arc;

use rust_simple_raytracer::{
    construct_planar_quad_box, Camera, CameraParams, Hittables, HittablesList, Lambertain,
    LightMaterials, Materials, Quad, ScatterMaterials, Vec3, BVH,
};

fn test_scene() {
    let red = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Lambertain(
        Lambertain {
            albedo: Vec3::new(0.65, 0.05, 0.05),
        },
    )));
    let white = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Lambertain(
        Lambertain {
            albedo: Vec3::new(0.73, 0.73, 0.73),
        },
    )));
    let green = Arc::new(Materials::ScatterMaterial(ScatterMaterials::Lambertain(
        Lambertain {
            albedo: Vec3::new(0.12, 0.45, 0.15),
        },
    )));
    let light = Arc::new(Materials::LightMaterial(LightMaterials::Diffuse {
        power: 15.0,
    }));

    let mut hittable_list = HittablesList::new();
    hittable_list.add(Arc::new(Hittables::Quad(Quad::new(
        Vec3::new_int(555, 0, 0),
        Vec3::new_int(0, 555, 0),
        Vec3::new_int(0, 0, 555),
        green.clone(),
    ))));
    hittable_list.add(Arc::new(Hittables::Quad(Quad::new(
        Vec3::new_int(0, 0, 0),
        Vec3::new_int(0, 555, 0),
        Vec3::new_int(0, 0, 555),
        red.clone(),
    ))));
    hittable_list.add(Arc::new(Hittables::Quad(Quad::new(
        Vec3::new_int(343, 554, 332),
        Vec3::new_int(-130, 0, 0),
        Vec3::new_int(0, 0, -105),
        light.clone(),
    ))));
    hittable_list.add(Arc::new(Hittables::Quad(Quad::new(
        Vec3::new_int(0, 0, 0),
        Vec3::new_int(555, 0, 0),
        Vec3::new_int(0, 0, 555),
        white.clone(),
    ))));
    hittable_list.add(Arc::new(Hittables::Quad(Quad::new(
        Vec3::new_int(555, 555, 555),
        Vec3::new_int(-555, 0, 0),
        Vec3::new_int(0, 0, -555),
        white.clone(),
    ))));
    hittable_list.add(Arc::new(Hittables::Quad(Quad::new(
        Vec3::new_int(0, 0, 555),
        Vec3::new_int(555, 0, 0),
        Vec3::new_int(0, 555, 0),
        white.clone(),
    ))));
    hittable_list.append(&mut construct_planar_quad_box(
        &Vec3::new_int(130, 0, 65),
        &Vec3::new_int(295, 165, 230),
        white.clone(),
    ));
    hittable_list.append(&mut construct_planar_quad_box(
        &Vec3::new_int(265, 0, 295),
        &Vec3::new_int(430, 330, 460),
        white.clone(),
    ));
    let world = BVH::from_hittable_list(&hittable_list);

    let camera_params = CameraParams {
        aspect_ratio: 1.0,
        samples_per_pixel: 100,
        max_depth: 100,
        image_width: 600,
        fov: 40_f64,
        focus_angle: 0_f64,
        look_from: Vec3::new_int(278, 278, -800),
        look_at: Vec3::new_int(278, 278, 0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        background: Vec3::new(0.0, 0.0, 0.0),
        // focus_distance: 0.4,
        ..Default::default()
    };
    let camera = Camera::new(camera_params);

    eprintln!("{:?}", camera);
    // eprintln!("{}", world);
    camera.render(&world);
}

fn main() {
    test_scene();
}
