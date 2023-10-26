use std::rc::Rc;

use crate::helper::Interval;
use crate::materials::Scatterable;
use crate::ray::Ray;
use crate::HitRecord;
use crate::Hittable;

use super::Materials;

use super::Vec3;

struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<Materials>,
}

impl Hittable for Sphere {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<Vec3> {
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
            root
        } else {
            (-b + sqrt_discriminant) / a
        };
        if !valid_t_interval.surrounds(root) {
            return None;
        }
        let outward_normal_unit = (_ray.at(root) - self.center.clone()).unit_vector();
        let hit_record = HitRecord::new(_ray, &outward_normal_unit, root);
        self.material.scatter(_ray, &hit_record)
    }
}

pub enum Objects {
    Sphere,
}
