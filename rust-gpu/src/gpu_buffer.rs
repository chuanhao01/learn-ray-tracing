use crate::Vec3f;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Material {
    t: u32,
    scatter_id: u32,
    emit_id: u32,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ScatterMaterial {
    t: u32,
    albedo: Vec3f,
    fuzzy_factor: f32,
    index_of_reflectance: f32,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Sphere {
    center: Vec3f,
    radius: f32,
}
