use std::fmt::Display;

use crate::{Interval, Ray, Vec3};

/// Axis Aligned Bounding Box
/// To initialize use `AABB{}` or [from_aabb] or [from_points]
#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}
impl AABB {
    /// Create the AABB given 2 points, with each axis covering from p1 to p2
    pub fn from_points(p1: &Vec3, p2: &Vec3) -> AABB {
        AABB {
            x: Interval {
                min: f64::min(p1.x(), p2.x()),
                max: f64::max(p1.x(), p2.x()),
            },
            y: Interval {
                min: f64::min(p1.y(), p2.y()),
                max: f64::max(p1.y(), p2.y()),
            },
            z: Interval {
                min: f64::min(p1.z(), p2.z()),
                max: f64::max(p1.z(), p2.z()),
            },
        }
    }
    /// Create a AABB from 2 AABB, containing both of the input AABB
    pub fn from_aabb(a: &Self, b: &Self) -> Self {
        AABB {
            x: Interval::from_interval(&a.x, &b.x),
            y: Interval::from_interval(&a.y, &b.y),
            z: Interval::from_interval(&a.z, &b.z),
        }
    }
    /// Translate the AABB by the given offset, returns a new [AABB]
    pub fn translate(&self, offset: Vec3) -> Self {
        Self {
            x: self.x.translate(offset.x()),
            y: self.y.translate(offset.y()),
            z: self.z.translate(offset.z()),
        }
    }
    /// Get a reference to the intervals in the AABB based on index
    // TODO: Find a better way to deal with people using non-sensical indexes other than 0-2
    // Idea was to use an enum, but having to deal with errors is meh
    pub fn axis(&self, axis: i64) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Tried to index AABB axis with index, {}", axis),
        }
    }
    /// Pads the AABB x, y, z intervals incase they get too small
    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        AABB {
            x: if self.x.size() >= delta {
                self.x
            } else {
                self.x.expand(delta)
            },
            y: if self.y.size() >= delta {
                self.y
            } else {
                self.y.expand(delta)
            },
            z: if self.z.size() >= delta {
                self.z
            } else {
                self.z.expand(delta)
            },
        }
    }

    /// Quick and cheaper check for if the ray will hit the AABB
    pub fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<Interval> {
        let mut modified_t_interval = valid_t_interval;
        for axis in 0..3 {
            let inv_b = 1_f64 / _ray.direction[axis];
            let a = _ray.origin[axis];
            let interval = self.axis(axis as i64);

            let t0 = (interval.min - a) * inv_b;
            let t1 = (interval.max - a) * inv_b;

            // Swap if required
            let (t0, t1) = if inv_b < 0_f64 { (t1, t0) } else { (t0, t1) };
            if t0 > modified_t_interval.min {
                modified_t_interval.min = t0;
            }
            if t1 < modified_t_interval.max {
                modified_t_interval.max = t1;
            }

            // Check if ray still hits within the AABB
            // Because if the t values do not overlap, there does not exists a common t for which the ray stays in the AABB (so hits it)
            if modified_t_interval.max <= modified_t_interval.min {
                return None;
            }
        }
        Some(modified_t_interval)
    }
}
impl Display for AABB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AABB(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::f64::INFINITY;

    #[test]
    fn test_aabb_default() {
        let bbox = AABB::default();
        assert_eq!(bbox.x.min, 0_f64);
        assert_eq!(bbox.y.min, 0_f64);
        assert_eq!(bbox.z.min, 0_f64);

        assert_eq!(bbox.x.max, 0_f64);
        assert_eq!(bbox.y.max, 0_f64);
        assert_eq!(bbox.z.max, 0_f64);
    }

    #[test]
    fn test_aabb_hit() {
        let aabb = AABB::from_points(&Vec3::new_int(1, 1, 1), &Vec3::new_int(3, 3, 3));
        let hit = aabb.hit(
            &Ray {
                origin: Vec3::new_int(0, 0, 0),
                direction: Vec3::new_int(1, 1, 1),
            },
            Interval {
                min: 0.001,
                max: INFINITY,
            },
        );

        assert!(matches!(hit, Some(_interval)));
        let hit = aabb.hit(
            &Ray {
                origin: Vec3::new_int(0, 0, 0),
                direction: Vec3::new_int(-1, 1, 1),
            },
            Interval {
                min: 0.001,
                max: INFINITY,
            },
        );
        assert!(hit.is_none());
    }
}
