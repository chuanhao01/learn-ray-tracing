use crate::Vec3;

mod color;
mod image;

pub use color::{CheckeredTexture, SolidColor, SpatialCheckeredTexture};
pub use image::Image;

/// Public Trait to implement a 2D texture onto any object
pub trait ColorTexture: Sync + Send {
    /// (u, v)=(x, y) represent the 2D cordinates on the texture
    /// p represent the point on the object itself in the world
    /// Returns a color Vec
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
