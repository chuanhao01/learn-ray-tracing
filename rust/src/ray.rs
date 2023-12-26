use std::fmt::Display;

use super::Vec3;

/// Represent a Ray in the equation: R = O + tD
/// With O = Origin and D = Direction
/// To get the point at float t, use [Ray::at]
pub struct Ray {
    /// Point of Origin for the Ray
    pub origin: Vec3,
    /// Vector direction of the Ray
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.clone() + t * self.direction.clone()
    }
}
impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ray:\nOrigin: {}\nDirection: {}",
            self.origin, self.direction
        )
    }
}
impl Clone for Ray {
    fn clone(&self) -> Self {
        Ray {
            origin: self.origin.clone(),
            direction: self.direction.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ray_at() {
        let r = Ray {
            origin: Vec3::new_int(0, 0, 0),
            direction: Vec3::new_int(1, 1, 1),
        };
        assert_eq!(r.at(3_f64), Vec3::new_int(3, 3, 3));
    }
}
