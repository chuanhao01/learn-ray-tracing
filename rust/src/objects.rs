use std::fmt::Debug;
use std::fmt::Display;

use crate::{helper::Interval, ray::Ray, HitRecord, Hittable, Vec3};

mod aabb;
mod disk;
mod quad;
mod sphere;
mod triangle;

pub use aabb::AABB;
pub use disk::Disk;
pub use quad::Quad;
pub use sphere::Sphere;
pub use triangle::Triangle;

pub trait HittableObject: Hittable<HitRecord> {
    fn bbox(&self) -> &AABB;
}

// Allow snake case for understanding the object reason
#[allow(non_snake_case)]
struct PlanarBase {
    /// Origin point on the plane
    pub Q: Vec3,
    /// u (left) Vector of the plane
    pub u: Vec3,
    /// v (up) Vector of the plane
    pub v: Vec3,
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
impl PlanarBase {
    fn new(Q: Vec3, u: Vec3, v: Vec3) -> Self {
        let n = Vec3::cross(&u, &v);
        let plane_unit_normal = Vec3::unit_vector(&n);
        Self {
            Q: Q.clone(),
            u: u.clone(),
            v: v.clone(),
            plane_unit_normal: plane_unit_normal.clone(),
            D: Vec3::dot(&plane_unit_normal, &Q),
            w: n.clone() / Vec3::dot(&n, &n),
        }
    }
    fn hit_plane(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<PlanarPlaneHit> {
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
        Some(PlanarPlaneHit { t, alpha, beta })
    }
}
impl Display for PlanarBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlanarBase(Q: {}, v: {}, u: {})", self.Q, self.u, self.v)
    }
}
struct PlanarPlaneHit {
    t: f64,
    alpha: f64,
    beta: f64,
}
trait PlanarObject {
    /// Checks if the given alpha and beta values, based on the plane bases and origin
    /// Lie within the planar object
    fn is_in_planar_object(&self, alpha: f64, beta: f64) -> bool;
}

pub enum Hittables {
    Sphere(Sphere),
    Quad(Quad),
    Triangle(Triangle),
    Disk(Disk),
    None,
}
impl Hittables {
    /// Quick accessor to get the hittable bbox
    pub fn bbox(&self) -> &AABB {
        match self {
            Hittables::Sphere(sphere) => sphere.bbox(),
            Hittables::Quad(quad) => quad.bbox(),
            Hittables::Triangle(triangle) => triangle.bbox(),
            Hittables::Disk(disk) => disk.bbox(),
            Hittables::None => &AABB {
                x: Interval {
                    min: 0_f64,
                    max: 0_f64,
                },
                y: Interval {
                    min: 0_f64,
                    max: 0_f64,
                },
                z: Interval {
                    min: 0_f64,
                    max: 0_f64,
                },
            },
        }
    }
}
impl Hittable<HitRecord> for Hittables {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        match self {
            Hittables::Sphere(sphere) => sphere.hit(_ray, valid_t_interval),
            Hittables::Quad(quad) => quad.hit(_ray, valid_t_interval),
            Hittables::Triangle(triangle) => triangle.hit(_ray, valid_t_interval),
            Hittables::Disk(disk) => disk.hit(_ray, valid_t_interval),
            Hittables::None => None,
        }
    }
}
impl Display for Hittables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let obj = match self {
            Hittables::Sphere(sphere) => sphere.to_string(),
            Hittables::Quad(quad) => quad.to_string(),
            Hittables::Triangle(triangle) => triangle.to_string(),
            Hittables::Disk(disk) => disk.to_string(),
            Hittables::None => "Nothing".to_owned(),
        };
        write!(f, "{}", obj)
    }
}
impl Debug for Hittables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use std::f64::INFINITY;

    use super::*;

    #[test]
    fn test_planar_disk_hit_plane() {
        let xy_plane = PlanarBase::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let negative_alpha_beta_ray = Ray {
            origin: Vec3::new(0.0, 0.0, -1.0),
            direction: Vec3::new(-3.0, -4.0, 1.0),
        };
        if let Some(negative_alpha_beta_hit) = xy_plane.hit_plane(
            &negative_alpha_beta_ray,
            Interval {
                min: 0.001,
                max: INFINITY,
            },
        ) {
            assert_eq!(negative_alpha_beta_hit.t, 1.0);
            assert_eq!(negative_alpha_beta_hit.alpha, -3.0);
            assert_eq!(negative_alpha_beta_hit.beta, -4.0);
        } else {
            panic!("Ray should hit planar base")
        };
    }
}
