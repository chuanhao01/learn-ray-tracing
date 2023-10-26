use crate::{ray::Ray, HitRecord};

use super::Vec3;

pub trait Scatterable {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Vec3>;
}

pub struct Lambertain {
    albedo: Vec3,
}

impl Scatterable for Lambertain {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Vec3> {
        None
    }
}

pub enum Materials {
    Lambertain(Lambertain),
}

impl Scatterable for Materials {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Vec3> {
        match self {
            Materials::Lambertain(lambertain) => lambertain.scatter(_ray, hit_record),
        }
    }
}
