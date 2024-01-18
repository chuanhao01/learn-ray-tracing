use std::sync::Arc;

use super::ColorTexture;
use crate::Vec3;

pub struct SolidColor {
    pub color: Vec3,
}
impl ColorTexture for SolidColor {
    fn color(&self, _: f64, _: f64, _: Vec3) -> Vec3 {
        self.color.clone()
    }
}

pub struct SpatialCheckeredTexture {
    scale: f64,
    even_texture: Arc<dyn ColorTexture>,
    odd_texture: Arc<dyn ColorTexture>,
}
impl SpatialCheckeredTexture {
    pub fn new(
        scale: f64,
        even_texture: Arc<dyn ColorTexture>,
        odd_texture: Arc<dyn ColorTexture>,
    ) -> Self {
        Self {
            scale: 1.0 / scale,
            even_texture,
            odd_texture,
        }
    }
    pub fn from_colors(scale: f64, even_color: Vec3, odd_color: Vec3) -> Self {
        Self {
            scale,
            even_texture: Arc::new(SolidColor { color: even_color }),
            odd_texture: Arc::new(SolidColor { color: odd_color }),
        }
    }
}
impl ColorTexture for SpatialCheckeredTexture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let scaled_floor_p = p
            .xyz()
            .iter()
            .fold(0_i64, |acc, v| acc + (v * self.scale).floor() as i64);
        if scaled_floor_p % 2 == 0 {
            self.even_texture.color(u, v, p)
        } else {
            self.odd_texture.color(u, v, p)
        }
    }
}

pub struct CheckeredTexture {
    scale: f64,
    even_texture: Arc<dyn ColorTexture>,
    odd_texture: Arc<dyn ColorTexture>,
}
impl CheckeredTexture {
    pub fn new(
        scale: f64,
        even_texture: Arc<dyn ColorTexture>,
        odd_texture: Arc<dyn ColorTexture>,
    ) -> Self {
        Self {
            scale: 1.0 / scale,
            even_texture,
            odd_texture,
        }
    }
    pub fn from_colors(scale: f64, even_color: Vec3, odd_color: Vec3) -> Self {
        Self {
            scale,
            even_texture: Arc::new(SolidColor { color: even_color }),
            odd_texture: Arc::new(SolidColor { color: odd_color }),
        }
    }
}
impl ColorTexture for CheckeredTexture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let scaled_floor_p = [u, v]
            .iter()
            .fold(0_i64, |acc, v| acc + (v * self.scale).floor() as i64);
        if scaled_floor_p % 2 == 0 {
            self.even_texture.color(u, v, p)
        } else {
            self.odd_texture.color(u, v, p)
        }
    }
}
