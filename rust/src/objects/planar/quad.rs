use std::fmt::Display;

use crate::{HitRecord, Hittable, HittableWithBBox, Interval, Materials, Ray, Vec3, AABB};

use super::{PlanarBase, PlanarObject};

pub struct Quad {
    planar_base: PlanarBase,
    pub material: Materials,
    bbox: AABB,
}
#[allow(non_snake_case)]
impl Quad {
    /// Q being the bottom left point
    /// u being the left pointing vector
    /// v being the up poiting vector
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, material: Materials) -> Self {
        Self {
            planar_base: PlanarBase::new(Q.clone(), u.clone(), v.clone()),
            material,
            // Important Note:
            // bbox requires padding as Some quads can lie on the axis (Size = 0)
            bbox: AABB::from_points(Q.clone(), Q.clone() + u.clone() + v.clone()).pad(),
        }
    }
}
impl PlanarObject for Quad {
    fn ab_is_in_planar_object(&self, alpha: f64, beta: f64) -> bool {
        let quad_interval = Interval { min: 0.0, max: 1.0 };
        quad_interval.contains(alpha) && quad_interval.contains(beta)
    }
}
impl HittableWithBBox for Quad {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
impl Hittable for Quad {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        let plane_hit = match self.planar_base.hit_plane(_ray, valid_t_interval) {
            Some(plane_hit) => plane_hit,
            None => {
                return None;
            }
        };
        // If the hit is not within the quad
        if !self.ab_is_in_planar_object(plane_hit.alpha, plane_hit.beta) {
            return None;
        }

        // Since the Quad is defined by u, v
        // The alpha and beta values if there is a hit on the quad is already mapped correctly [0. 1]
        Some(HitRecord::new(
            _ray,
            &self.planar_base.plane_unit_normal,
            plane_hit.t,
            self.material.clone(),
            plane_hit.alpha,
            plane_hit.beta,
        ))
    }
}
impl Display for Quad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quad(planar_base: {})", self.planar_base)
    }
}
