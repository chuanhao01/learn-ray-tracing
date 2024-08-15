use crate::Vec3f;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Material {
    t: u32,
    scatter_idx: u32,
    emit_idx: u32,
}
impl Material {
    pub fn new(t: u32, scatter_idx: u32, emit_idx: u32) -> Self {
        Self {
            t,
            scatter_idx,
            emit_idx,
        }
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C, align(16))] // Align 16 because of vec3f
pub struct ScatterMaterial {
    albedo: Vec3f,
    t: u32,
    fuzzy_factor: f32,
    index_of_reflectance: f32,
    _padding: [u32; 2],
}
impl ScatterMaterial {
    pub fn new(albedo: Vec3f, t: u32, fuzzy_factor: f32, index_of_reflectance: f32) -> Self {
        Self {
            albedo,
            t,
            fuzzy_factor,
            index_of_reflectance,
            _padding: [0u32; 2],
        }
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct EmitMaterial {
    t: u32,
    power: f32,
}
impl EmitMaterial {
    pub fn new(t: u32, power: f32) -> Self {
        Self { t, power }
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C, align(16))] // Align 16 because of vec3f
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

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C, align(16))]
/// Refer to crate::common::Camera
pub struct CameraUniform {
    // 16
    look_from: Vec3f,
    _pad0: u32,
    // 16
    look_at: Vec3f,
    _pad1: u32,
    // 16
    v_up: Vec3f,
    theta: f32,
}
