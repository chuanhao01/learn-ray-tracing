use std::f64::consts::PI;

use super::Vec3;

/// Converts a degree value into radians which we use internally
pub fn from_fdegree_to_fradian(degree: f64) -> f64 {
    degree * PI / 180_f64
}

/// Takes a float color vector and return the rgb int values as a tuple
pub fn color_to_rgb(color: &Vec3, samples_per_pixel: i64) -> (i64, i64, i64) {
    let c = color.clone();
    let scale = 1_f64 / samples_per_pixel as f64;
    let c = c * scale;
    let (r, g, b) = c.tuple();

    let r = r.sqrt();
    let g = g.sqrt();
    let b = b.sqrt();

    (
        (r.clamp(0.0, 0.999) * 256_f64) as i64,
        (g.clamp(0.0, 0.999) * 256_f64) as i64,
        (b.clamp(0.0, 0.999) * 256_f64) as i64,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_degree() {
        assert_eq!(from_fdegree_to_fradian(90_f64), PI / 2_f64);
        assert_eq!(from_fdegree_to_fradian(180_f64), PI);
        assert_eq!(from_fdegree_to_fradian(270_f64), PI * (3_f64 / 2_f64));
        assert_eq!(from_fdegree_to_fradian(360_f64), PI * 2_f64);
    }

    #[test]
    fn test_color_to_rgb() {
        let rgb = color_to_rgb(&Vec3::new_int(1, 1, 1), 1);
        assert_eq!(rgb, (255, 255, 255));

        let rgb = color_to_rgb(&Vec3::new_int(0, 0, 0), 1);
        assert_eq!(rgb, (0, 0, 0));

        let rgb = color_to_rgb(&Vec3::new(0.5, 0.5, 0.5), 1);
        assert_eq!(rgb, (181, 181, 181));
    }
}
