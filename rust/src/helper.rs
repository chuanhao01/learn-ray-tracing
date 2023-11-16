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

/// Simple structure for representing Intervals
///
/// There is a way to do it in rust with std::ops::Range,
/// but I don't know and truct myself to use it properly
#[derive(Default, Clone, Copy)]
pub struct Interval {
    /// Left bound
    pub min: f64,
    /// Right bound
    pub max: f64,
}

impl Interval {
    /// Creates an interval that encapsulates both input intervals (i.e. a larger than both intervals)
    pub fn from_interval(a: &Self, b: &Self) -> Self {
        Self {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
    }

    /// returns the size of the interval
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    /// Expands the interval with the given delta, lowering the min by half and increasing the max by half
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2_f64;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
    /// Checks if provided x is `l <= x <= r`
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    /// Checks if provided x is `l < x < r`
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
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

    #[test]
    fn test_interval_default() {
        let i = Interval::default();
        assert_eq!(i.min, 0_f64);
        assert_eq!(i.max, 0_f64);
    }

    #[test]
    fn test_interval_expand() {
        let i = Interval { min: 0.0, max: 0.0 };
        let i = i.expand(1.0);
        assert_eq!(i.min, -0.5);
        assert_eq!(i.max, 0.5);
    }
}
