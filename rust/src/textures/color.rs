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
