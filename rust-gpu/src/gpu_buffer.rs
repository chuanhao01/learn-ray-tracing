use crate::Vec3f;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Material {
    t: u32,
    scatter_idx: u32,
    emit_idx: u32,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct ScatterMaterial {
    t: u32,
    albedo: Vec3f,
    fuzzy_factor: f32,
    index_of_reflectance: f32,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Sphere {
    center: Vec3f,
    radius: f32,
    material_idx: u32,
}
impl Sphere {
    pub fn new(center: Vec3f, radius: f32, material_idx: u32) -> Self {
        Sphere {
            center,
            radius,
            material_idx,
        }
    }
}
