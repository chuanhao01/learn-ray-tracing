use std::fmt::Debug;
use std::fmt::Display;

use crate::{helper::Interval, ray::Ray, HitRecord, Hittable};

mod aabb;
mod quad;
mod sphere;

pub use aabb::AABB;
pub use quad::Quad;
pub use sphere::Sphere;

pub trait HittableObject: Hittable<HitRecord> {
    fn bbox(&self) -> &AABB;
}
trait PlanarObject {
    /// Checks if the given alpha and beta values, based on the plane bases and origin
    /// Lie within the planar object
    fn is_in_planar_object(alpha: f64, beta: f64) -> bool;
}

pub enum Hittables {
    Sphere(Sphere),
    Quad(Quad),
    None,
}
impl Hittables {
    /// Quick accessor to get the hittable bbox
    pub fn bbox(&self) -> &AABB {
        match self {
            Hittables::Sphere(sphere) => sphere.bbox(),
            Hittables::Quad(quad) => quad.bbox(),
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
            Hittables::None => None,
        }
    }
}
impl Display for Hittables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let obj = match self {
            Hittables::Sphere(sphere) => sphere.to_string(),
            Hittables::Quad(quad) => quad.to_string(),
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
mod test {}
