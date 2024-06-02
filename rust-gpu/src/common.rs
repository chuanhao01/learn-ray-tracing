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
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }
}
