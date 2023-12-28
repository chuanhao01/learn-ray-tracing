use std::fmt::Debug;
use std::fmt::Display;

use crate::{helper::Interval, ray::Ray, HitRecord, Hittable};

mod aabb;
mod planar;
mod rotation;
mod simple_constructors;
mod sphere;
mod translation;

pub use aabb::AABB;
pub use planar::{Disk, Quad, Triangle};
pub use rotation::Rotation;
pub use simple_constructors::construct_planar_quad_box;
pub use sphere::Sphere;
pub use translation::Translation;

pub trait HittableObject: Hittable<HitRecord> {
    fn bbox(&self) -> &AABB;
}

pub enum Hittables {
    Sphere(Sphere),
    Quad(Quad),
    Triangle(Triangle),
    Disk(Disk),
    Translation(Translation),
    Rotation(Rotation),
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
            Hittables::Translation(translation) => translation.bbox(),
            Hittables::Rotation(rotation) => rotation.bbox(),
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
            Hittables::Translation(translation) => translation.hit(_ray, valid_t_interval),
            Hittables::Rotation(rotation) => rotation.hit(_ray, valid_t_interval),
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
            Hittables::Translation(translation) => translation.to_string(),
            Hittables::Rotation(rotation) => rotation.to_string(),
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
