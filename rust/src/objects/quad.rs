use std::{fmt::Display, sync::Arc};

use crate::{HitRecord, Hittable, Interval, Materials, Ray, Vec3, AABB};

use super::{HittableObject, PlanarObject};

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
    // Note: The implicit formula for the Plane is based on the plane's unit normal to calculate D
    /// Normal of the plane the Quad is in
    plane_unit_normal: Vec3,
    /// D value based on the plane implicit Equation Ax + By + Cz = D
    D: f64,
    /// Memorise w vector, which is normal / normal . normal (normal = u x v)
    /// Not to be confused with the plane's unit_normal
    w: Vec3,
}
#[allow(non_snake_case)]
impl Quad {
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, material: Arc<Materials>) -> Self {
        let n = Vec3::cross(&u, &v);
        let plane_unit_normal = Vec3::unit_vector(&n);
        Self {
            Q: Q.clone(),
            u: u.clone(),
            v: v.clone(),
            material,
            // Important Note:
            // bbox requires padding as Some quads can lie on the axis (Size = 0)
            bbox: AABB::from_points(&Q.clone(), &(Q.clone() + u.clone() + v.clone())).pad(),
            plane_unit_normal: plane_unit_normal.clone(),
            D: Vec3::dot(&plane_unit_normal, &Q),
            w: n.clone() / Vec3::dot(&n, &n),
        }
    }
}
impl PlanarObject for Quad {
    fn is_in_planar_object(alpha: f64, beta: f64) -> bool {
        let quad_interval = Interval { min: 0.0, max: 1.0 };
        quad_interval.contains(alpha) && quad_interval.contains(beta)
    }
}
impl HittableObject for Quad {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
impl Hittable<HitRecord> for Quad {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        // Hit implementation based on plane_unit_normal and incoming Ray
        let denom = Vec3::dot(&self.plane_unit_normal, &_ray.direction);

        // If the direction of the ray is ever parallel to the plane the Quad is on, it does not hit
        if denom.abs() < 1e-8_f64 {
            return None;
        }

        // Ray will hit the plane, checking if within t interval
        let t = (self.D - Vec3::dot(&self.plane_unit_normal, &_ray.origin)) / denom;
        if !valid_t_interval.surrounds(t) {
            return None;
        }

        // Calculating the alpha, beta values with u and v as bases and origin at Q
        let intersection = _ray.at(t);
        let p = intersection - self.Q.clone();
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&p, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &p));
        // If the hit is not within the quad
        if !Quad::is_in_planar_object(alpha, beta) {
            return None;
        }

        Some(HitRecord::new(
            _ray,
            &self.plane_unit_normal,
            t,
            self.material.clone(),
        ))
    }
}
impl Display for Quad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quad(Q: {}, u: {}, v: {})", self.Q, self.u, self.v)
    }
}
