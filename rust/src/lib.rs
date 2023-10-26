pub mod camera;
pub mod helper;
pub mod hittable;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod vec3;

pub use camera::{Camera, CameraParams};
pub use helper::Interval;
pub use hittable::{HitRecord, Hittable};
pub use materials::{Materials, Scattered, Lambertain};
pub use objects::{Hittables, Sphere};
pub use vec3::Vec3;
