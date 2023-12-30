use std::sync::Arc;

use crate::{HittableWithBBox, Materials, Quad, Vec3, BVH};

/// a and b are the bottom and top point of the box
/// Material will be the material of all the quads of the sides of the box
/// The points a and b will then be converted into the bottom left and top right points of the box
pub fn construct_planar_quad_box(a: &Vec3, b: &Vec3, material: Arc<Materials>) -> BVH {
    // Since the BVH takes in a Vec<Arc> of Hittables
    let mut box_quads: Vec<Arc<dyn HittableWithBBox>> = Vec::new();
    let bottom_left_point = Vec3::new(
        f64::min(a.x(), b.x()),
        f64::min(a.y(), b.y()),
        f64::min(a.z(), b.z()),
    );
    let top_right_point = Vec3::new(
        f64::max(a.x(), b.x()),
        f64::max(a.y(), b.y()),
        f64::max(a.z(), b.z()),
    );

    let dx = Vec3::new(top_right_point.x() - bottom_left_point.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, top_right_point.y() - bottom_left_point.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, top_right_point.z() - bottom_left_point.z());

    box_quads.push(Arc::new(Quad::new(
        bottom_left_point.clone(),
        dx.clone(),
        dy.clone(),
        material.clone(),
    ))); // Front face
    box_quads.push(Arc::new(Quad::new(
        bottom_left_point.clone(),
        dz.clone(),
        dy.clone(),
        material.clone(),
    ))); // Left Side
    box_quads.push(Arc::new(Quad::new(
        bottom_left_point.clone(),
        dx.clone(),
        dz.clone(),
        material.clone(),
    ))); // Bottom Side
    box_quads.push(Arc::new(Quad::new(
        top_right_point.clone(),
        -dx.clone(),
        -dy.clone(),
        material.clone(),
    ))); // Back face
    box_quads.push(Arc::new(Quad::new(
        top_right_point.clone(),
        -dz.clone(),
        -dy.clone(),
        material.clone(),
    ))); // Right Side
    box_quads.push(Arc::new(Quad::new(
        top_right_point.clone(),
        -dx.clone(),
        -dz.clone(),
        material.clone(),
    ))); // Back face
    BVH::from_hittables_list(box_quads)
}
