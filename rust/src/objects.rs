use std::rc::Rc;

use crate::helper::Interval;
use crate::ray::Ray;
use crate::HitRecord;
use crate::Hittable;

use super::Materials;

use super::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Materials>,
}

impl Hittable for Sphere {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        let a_minus_c = _ray.origin.clone() - self.center.clone();

        let a = _ray.direction.length_squared();
        let b = Vec3::dot(&a_minus_c, &_ray.direction);
        let c = a_minus_c.length_squared() - self.radius * self.radius;

        // Ray does not hit the sphere
        let discriminant = b * b - a * c;
        if discriminant < 0_f64 {
            return None;
        }

        // Find the closer root, since the ray is from the camera, smaller t is closer to the camera
        let sqrt_discriminant = discriminant.sqrt();
        let root = (-b - sqrt_discriminant) / a;
        let root = if !valid_t_interval.surrounds(root) {
            (-b + sqrt_discriminant) / a
        } else {
            root
        };
        if !valid_t_interval.surrounds(root) {
            return None;
        }
        let outward_normal_unit = (_ray.at(root) - self.center.clone()).unit_vector();
        Some(HitRecord::new(
            _ray,
            &outward_normal_unit,
            root,
            Rc::clone(&self.material),
        ))
    }
}

pub enum Hittables {
    Sphere(Sphere),
}

impl Hittable for Hittables {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        match self {
            Hittables::Sphere(sphere) => sphere.hit(_ray, valid_t_interval),
        }
    }
}
