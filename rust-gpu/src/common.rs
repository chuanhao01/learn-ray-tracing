use std::ops::Add;

use bytemuck::{Pod, Zeroable};

// Common vec used both in rust and memmory safe for buffer use in wgsl
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }
}
impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Self) -> Self::Output {}
}

pub struct Camera {
    /// Origin
    look_from: Vec3f,
    look_at: Vec3f,
    /// "General" up direction
    v_up: Vec3f,
    theta: f32,
    // Basis Vectors
    u: Vec3f, // Right
    v: Vecf,  // Up
    w: Vec3f, // Opposite of look from - look at
}
impl Camera {
    pub fn new(look_from: Vec3f, look_at: Vec3f, v_up: Vec3f, theta: f32) {
        let _w = look_at - look_from;
    }
}

pub struct InitParam {
    pub vp_width: u32,
    pub aspect_ratio: f32,
    pub camera_theta: f32, // In Degrees
    pub look_at: Vec3f,
    pub look_from: Vec3f,
    pub v_up: Vec3f,
}
impl Default for InitParam {
    fn default() -> Self {
        Self {
            vp_width: 1000u32,
            aspect_ratio: 16f32 / 9f32,
            camera_theta: 70f32,
            look_at: Vec3f::new(0f32, 0f32, -1f32),
            look_from: Vec3f::new(0f32, 0f32, 0f32),
            v_up: Vec3f::new(0f32, 1f32, 0f32),
        }
    }
}

pub struct InitConfig {
    pub vp_width: u32,
    pub vp_height: u32,
    pub camera_theta: f32, // In Degrees
    pub look_at: Vec3f,
    pub look_from: Vec3f,
    pub v_up: Vec3f,
}
impl InitConfig {
    pub fn new(init_param: InitParam) -> Self {
        Self {
            vp_width: init_param.vp_width,
            vp_height: (init_param.vp_width as f32 / init_param.aspect_ratio) as u32,
            camera_theta: init_param.camera_theta,
            look_at: init_param.look_at,
            look_from: init_param.look_from,
            v_up: init_param.v_up,
        }
    }
}
