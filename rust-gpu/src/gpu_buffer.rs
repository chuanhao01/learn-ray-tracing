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
    albedo: Vec3f,
    t: u32,
    fuzzy_factor: f32,
    index_of_reflectance: f32,
    _padding: u32,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Sphere {
    center: Vec3f,
    radius: f32,
    material_idx: u32,
    _padding: [u32; 3],
}
impl Sphere {
    pub fn new(center: Vec3f, radius: f32, material_idx: u32) -> Self {
        Sphere {
            center,
            radius,
            material_idx,
            _padding: [0u32; 3],
        }
    }
}
