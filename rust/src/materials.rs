use crate::{ray::Ray, HitRecord};

use super::Vec3;

pub struct Scattered {
    pub attenuation: Vec3,
    pub ray: Ray,
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
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    /// Ratio to scale the sampled unit circle, for the reflected ray + fuzziness
    fuzzy_factor: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzzy_factor: f64) -> Metal {
        Metal {
            albedo,
            fuzzy_factor: if fuzzy_factor < 1_f64 {
                fuzzy_factor
            } else {
                1_f64
            },
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let scattered_direction = Vec3::reflect(
            &_ray.direction.unit_vector(),
            &hit_record.against_normal_unit,
        ) + self.fuzzy_factor * Vec3::random_vector_in_unit_sphere();
        // Check if the scattered rays are cancelled out or scattered below the surface, in that case, ray is absorbed
        if Vec3::dot(&scattered_direction, &hit_record.against_normal_unit) > 0_f64 {
            Some(Scattered {
                attenuation: self.albedo.clone(),
                ray: Ray {
                    origin: hit_record.p.clone(),
                    direction: scattered_direction,
                },
            })
        } else {
            None
        }
    }
}

pub enum Materials {
    Lambertain(Lambertain),
    Metal(Metal),
    None,
}

impl Scatterable for Materials {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        match self {
            Materials::Lambertain(lambertain) => lambertain.scatter(_ray, hit_record),
            Materials::Metal(metal) => metal.scatter(_ray, hit_record),
            Materials::None => None,
        }
    }
}
