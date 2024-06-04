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

pub struct InitParam {
    pub vp_width: u32,
    pub aspect_ratio: f32,
    pub camera_focal_distance: f32,
    pub camera_theta: f32, // In Degrees
}
impl Default for InitParam {
    fn default() -> Self {
        Self {
            vp_width: 1000u32,
            aspect_ratio: 16f32 / 9f32,
            camera_focal_distance: 1f32,
            camera_theta: 70f32,
        }
    }
}

pub struct InitConfig {
    pub vp_width: u32,
    pub vp_height: u32,
    pub camera_focal_distance: f32,
    pub camera_theta: f32, // In Degrees
}
impl InitConfig {
    pub fn new(init_param: InitParam) -> Self {
        Self {
            vp_width: init_param.vp_width,
            vp_height: (init_param.vp_width as f32 / init_param.aspect_ratio) as u32,
            camera_focal_distance: init_param.camera_focal_distance,
            camera_theta: init_param.camera_theta,
        }
    }
}
