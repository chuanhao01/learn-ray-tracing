use std::{fmt::Display, sync::Arc};

use crate::{HitRecord, Hittable, Interval, Materials, Ray, Vec3, AABB};

use super::{HittableObject, PlanarBase, PlanarObject};

pub struct Disk {
    planar_base: PlanarBase,
    pub material: Arc<Materials>,
    bbox: AABB,
    radius: f64,
}
#[allow(non_snake_case)]
impl Disk {
    /// Q being the center of the circle
    /// u being the left pointing vector (in relation to the plane, will be converted to a unit vector)
    /// v being the up poiting vector (in relation to the plane, will be converted to a unit vector)
    /// raidus being the radius of the disk (In terms of unit vectors in the u and v vector directions)
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, radius: f64, material: Arc<Materials>) -> Self {
        let u = Vec3::unit_vector(&u);
        let v = Vec3::unit_vector(&v);

        let bottom_left_corner = Q.clone() - radius * u.clone() - radius * v.clone();
        let top_left_corner = Q.clone() + radius * u.clone() + radius * v.clone();
        Self {
            planar_base: PlanarBase::new(Q.clone(), u.clone(), v.clone()),
            material,
            // Important Note:
            // bbox requires padding as Some quads can lie on the axis (Size = 0)
            bbox: AABB::from_points(&bottom_left_corner, &top_left_corner).pad(),
            radius,
        }
    }
}
impl PlanarObject for Disk {
    fn ab_is_in_planar_object(&self, alpha: f64, beta: f64) -> bool {
        (alpha * alpha + beta * beta) <= (self.radius * self.radius)
    }
}
impl HittableObject for Disk {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
impl Hittable<HitRecord> for Disk {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        let plane_hit = match self.planar_base.hit_plane(_ray, valid_t_interval) {
            Some(plane_hit) => plane_hit,
            None => {
                return None;
            }
        };
        if !self.ab_is_in_planar_object(plane_hit.alpha, plane_hit.beta) {
            return None;
        }

        Some(HitRecord::new(
            _ray,
            &self.planar_base.plane_unit_normal,
            plane_hit.t,
            self.material.clone(),
        ))
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Disk(planar_base: {}, raidus: {})",
            self.planar_base, self.radius
        )
    }
}
