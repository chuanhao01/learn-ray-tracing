use std::{fmt::Display, sync::Arc};

use crate::{HitRecord, Hittable, Interval, Materials, Ray, Vec3, AABB};

use super::HittableObject;

// Allow snake case for understanding the object reason
#[allow(non_snake_case)]
pub struct Quad {
    /// Bottom left point of the quad
    pub Q: Vec3,
    /// u Vector of the quad (Right facing)
    pub u: Vec3,
    /// v Vector of the quad (Up facing)
    pub v: Vec3,
    pub material: Arc<Materials>,
    bbox: AABB,
}
#[allow(non_snake_case)]
impl Quad {
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, material: Arc<Materials>) -> Self {
        Self {
            Q: Q.clone(),
            u: u.clone(),
            v: v.clone(),
            material,
            // Important Note:
            // bbox requires padding as Some quads can lie on the axis (Size = 0)
            bbox: AABB::from_points(&Q.clone(), &(Q.clone() + u.clone() + v.clone())).pad(),
        }
    }
}
impl HittableObject for Quad {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
impl Hittable<HitRecord> for Quad {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        None
    }
}
impl Display for Quad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quad(Q: {}, u: {}, v: {})", self.Q, self.u, self.v)
    }
}
