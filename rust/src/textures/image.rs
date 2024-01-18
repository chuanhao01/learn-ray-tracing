use std::sync::Arc;

use image::{io::Reader as ImageReader, GenericImageView, RgbImage};

use super::ColorTexture;
use crate::{rgb_to_color, Interval, SolidColor, Vec3};

pub struct Image {
    image: RgbImage,
    inv_scale: f64,
    fill_texture: Arc<dyn ColorTexture>,
}
impl Image {
    /// For repeating textures, turn down the scale of the image on the object (<1.0)
    /// For zoomed in textures, turn up the scale (>1.0)
    /// For a normal texture wrapper make the scale (1.0)
    /// All objects image cordinates (u, v) should be already normalized to between [0, 1]
    pub fn new(scale: f64, image_path: &str, fill_texture: Arc<dyn ColorTexture>) -> Self {
        let image = match ImageReader::open(image_path) {
            Ok(opened_image) => match opened_image.decode() {
                Ok(image) => {
                    let (width, height) = image.dimensions();
                    if width == 0 || height == 0 {
                        panic!("Image has no dimensions");
                    }
                    image.to_rgb8()
                }
                Err(err) => panic!("Could not decode image\n{}", err),
            },
            Err(err) => panic!("Could not open image_path: {}\n{}", image_path, err),
        };
        Self {
            image,
            inv_scale: 1.0 / scale,
            fill_texture,
        }
    }
    pub fn new_with_color(scale: f64, image_path: &str, color: Vec3) -> Self {
        Self::new(scale, image_path, Arc::new(SolidColor { color }))
    }
}
impl ColorTexture for Image {
    // TODO: Fix issue with flipped v for y
    // Not working
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let mut u = u;
        let mut v = v;
        u *= self.inv_scale;
        v *= self.inv_scale;

        let valid_uv = Interval { min: 0.0, max: 1.0 };
        if valid_uv.contains(u) && valid_uv.contains(v) {
            // eprintln!("{:.2}, {:.2}", u, v);
            let [r, g, b] = self
                .image
                .get_pixel(
                    (u * self.image.width() as f64) as u32,
                    ((1.0 - v) * self.image.height() as f64) as u32,
                )
                .0;
            rgb_to_color(r, g, b)
        } else {
            self.fill_texture.color(u, v, p)
        }
    }
}
