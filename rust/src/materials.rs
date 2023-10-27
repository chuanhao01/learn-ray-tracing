use crate::{ray::Ray, HitRecord};

use super::Vec3;

pub struct Scattered {
    pub attenuation: Vec3,
    pub ray: Ray,
    /// t value of the ray used to scatter
    pub t: f64,
}

pub trait Scatterable {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered>;
}

pub struct Lambertain {
    pub albedo: Vec3,
}

impl Scatterable for Lambertain {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let scattered_direction =
            hit_record.against_normal_unit.clone() + Vec3::random_vector_in_unit_sphere();
        let scattered_direction = if scattered_direction.near_zero() {
            hit_record.against_normal_unit.clone()
        } else {
            scattered_direction
        };
        Some(Scattered {
            attenuation: self.albedo.clone(),
            ray: Ray {
                origin: hit_record.p.clone(),
                direction: scattered_direction,
            },
            t: hit_record.t,
        })
    }
}

pub enum Materials {
    Lambertain(Lambertain),
}

impl Scatterable for Materials {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        match self {
            Materials::Lambertain(lambertain) => lambertain.scatter(_ray, hit_record),
        }
    }
}
