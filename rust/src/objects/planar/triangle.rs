use std::fmt::Display;

use crate::{HitRecord, Hittable, HittableWithBBox, Interval, Materials, Ray, Vec3, AABB};

use super::{PlanarBase, PlanarObject};

// Allow snake case for understanding the object reason
#[allow(non_snake_case)]
pub struct Triangle {
    planar_base: PlanarBase,
    pub material: Materials,
    bbox: AABB,
}
#[allow(non_snake_case)]
impl Triangle {
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
impl PlanarObject for Triangle {
    fn ab_is_in_planar_object(&self, alpha: f64, beta: f64) -> bool {
        // If its no within this quad, its not a correct hit already
        let quad_interval = Interval { min: 0.0, max: 1.0 };
        if !(quad_interval.contains(alpha) && quad_interval.contains(beta)) {
            return false;
        }
        // Using the area check
        // The sum of the 3 triangles made with the new point should be the original area of the triangle
        let triangle_area = 0.5_f64;
        let left_triangle_area = (0.5_f64 * alpha).abs();
        let bottom_triangle_area = (0.5_f64 * beta).abs();
        // Watch out for the sign of the triangle
        let right_triangle_area = 0.5_f64
            * Vec3::cross(
                &Vec3::new(-alpha, 1.0 - beta, 0.0),
                &Vec3::new(1.0 - alpha, -beta, 0.0),
            )
            .z()
            .abs();

        (left_triangle_area + bottom_triangle_area + right_triangle_area - triangle_area).abs()
            < 1e-8_f64
    }
}
impl HittableWithBBox for Triangle {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
impl Hittable for Triangle {
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

        // TODO: Object Cordinates (u, v) crop out the image, since the triangle is only half
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
impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle(planar_base: {})", self.planar_base)
    }
}
