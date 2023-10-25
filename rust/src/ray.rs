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
    pub fn at(&self) -> Vec3 {
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
