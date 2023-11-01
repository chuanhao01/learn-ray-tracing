use std::sync::Arc;

use crate::helper::Interval;
use crate::ray::Ray;
use crate::HitRecord;
use crate::Hittable;

use super::Materials;

use super::Vec3;

/// Simple Sphere object
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Materials>,
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
        // let outward_normal_unit = (_ray.at(root) - self.center.clone()).unit_vector();
        let outward_normal_unit = (_ray.at(root) - self.center.clone()) / self.radius;
        Some(HitRecord::new(
            _ray,
            &outward_normal_unit,
            root,
            Arc::clone(&self.material),
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

#[cfg(test)]
mod test {
    use std::f64::INFINITY;

    use super::*;

    #[test]
    fn test_sphere_hit() {
        // Ensure the ray hits the sphere
        let mat = Arc::new(Materials::None);
        let s = Sphere {
            center: Vec3::new_int(0, 0, 0),
            radius: 1.0,
            material: Arc::clone(&mat),
        };
        let r = Ray {
            direction: Vec3::new_int(0, 0, 1),
            origin: Vec3::new_int(0, 0, -2),
        };
        let hr = s
            .hit(
                &r,
                Interval {
                    l: 0.001,
                    r: INFINITY,
                },
            )
            .unwrap();
        assert_eq!(hr.p, Vec3::new_int(0, 0, -1));
        assert_eq!(hr.t, 1.0);
        assert_eq!(hr.against_normal_unit, Vec3::new_int(0, 0, -1));
        assert!(Arc::ptr_eq(&mat, &hr.material));
        assert!(hr.front_face);

        // Ensure you get the second t value
        let hr = s
            .hit(
                &r,
                Interval {
                    l: 1.0,
                    r: INFINITY,
                },
            )
            .unwrap();
        assert_eq!(hr.t, 3.0);

        // Ensure interval out of range
        assert!(s.hit(&r, Interval { l: 4.0, r: 5.0 },).is_none());

        // Ensure that the ray does not hit the sphere
        let r = Ray {
            direction: Vec3::new_int(2, 0, 1),
            ..r
        };
        assert!(s
            .hit(
                &r,
                Interval {
                    l: 0.001,
                    r: INFINITY,
                },
            )
            .is_none());
    }
}
