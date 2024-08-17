pub mod common;
pub mod gpu_buffer;
pub mod materials;
pub mod render;
pub mod scene;

pub use common::{InitConfig, InitParam, Vec3f};
pub use render::PathTracer;
