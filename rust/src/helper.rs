use std::f64::consts::PI;

use super::Vec3;

/// Converts a degree value into radians which we use internally
pub fn from_fdegree_to_fradian(degree: f64) -> f64 {
    degree * PI / 180_f64
}

pub fn write_color(color: &Vec3, samples_per_pixel: i64) {
    let c = color.clone();
    let scale = 1_f64 / samples_per_pixel as f64;
    let c = c * scale;
    let (r, g, b) = c.tuple();

    let r = r.sqrt();
    let g = g.sqrt();
    let b = b.sqrt();

    println!(
        "{} {} {}",
        (r.clamp(0.0, 0.999) * 256_f64) as i64,
        (g.clamp(0.0, 0.999) * 256_f64) as i64,
        (b.clamp(0.0, 0.999) * 256_f64) as i64
    );
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
}
