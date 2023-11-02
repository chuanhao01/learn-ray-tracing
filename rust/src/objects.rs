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
    pub bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<Materials>) -> Self {
        let radius_v = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(
            &(center.clone() - radius_v.clone()),
            &(center.clone() + radius_v.clone()),
        );
        Self {
            center,
            radius,
            material,
            bbox,
        }
    }
}

impl Hittable<HitRecord> for Sphere {
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

/// Axis Aligned Bounding Box
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    /// Create the AABB given 2 points, with each axis covering from p1 to p2
    pub fn from_points(p1: &Vec3, p2: &Vec3) -> AABB {
        AABB {
            x: Interval {
                min: f64::min(p1.x(), p2.x()),
                max: f64::max(p1.x(), p2.x()),
            },
            y: Interval {
                min: f64::min(p1.y(), p2.y()),
                max: f64::max(p1.y(), p2.y()),
            },
            z: Interval {
                min: f64::min(p1.z(), p2.z()),
                max: f64::max(p1.z(), p2.z()),
            },
        }
    }
    /// Get a reference to the intervals in the AABB
    /// TODO: Find a better way to deal with people using non-sensical indexes other than 0-2
    pub fn axis(&self, axis: i64) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
}

impl Hittable<Interval> for AABB {
    /// Quick and cheaper check for if the ray will hit the AABB
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<Interval> {
        let mut modified_t_interval = valid_t_interval;
        for axis in 0..3 {
            let inv_b = 1_f64 / _ray.direction[axis];
            let a = _ray.origin[axis];
            let interval = self.axis(axis as i64);

            let t0 = (interval.min - a) * inv_b;
            let t1 = (interval.max - a) * inv_b;

            // Swap if required
            let (t0, t1) = if inv_b < 0_f64 { (t1, t0) } else { (t0, t1) };
            if t0 > modified_t_interval.min {
                modified_t_interval.min = t0;
            }
            if t1 < modified_t_interval.max {
                modified_t_interval.max = t1;
            }

            // Check if ray still hits within the AABB
            // Because if the t values do not overlap, there does not exists a common t for which the ray stays in the AABB (so hits it)
            if modified_t_interval.max < modified_t_interval.min {
                return None;
            }
        }
        Some(modified_t_interval)
    }
}

pub enum Hittables {
    Sphere(Sphere),
}

impl Hittable<HitRecord> for Hittables {
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
    fn test_sphere_new(){
        let mat = Arc::new(Materials::None);
        let s = Sphere::new(Vec3::new_int(0, 0, 0), 1.0, Arc::clone(&mat));
        assert_eq!(s.bbox.x.min, -1.0);
        assert_eq!(s.bbox.y.min, -1.0);
        assert_eq!(s.bbox.z.min, -1.0);

        assert_eq!(s.bbox.x.max, 1.0);
        assert_eq!(s.bbox.y.max, 1.0);
        assert_eq!(s.bbox.z.max, 1.0);
    }

    #[test]
    fn test_sphere_hit() {
        // Ensure the ray hits the sphere
        let mat = Arc::new(Materials::None);
        let s = Sphere::new(Vec3::new_int(0, 0, 0), 1.0, Arc::clone(&mat));
        let r = Ray {
            direction: Vec3::new_int(0, 0, 1),
            origin: Vec3::new_int(0, 0, -2),
        };
        let hr = s
            .hit(
                &r,
                Interval {
                    min: 0.001,
                    max: INFINITY,
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
                    min: 1.0,
                    max: INFINITY,
                },
            )
            .unwrap();
        assert_eq!(hr.t, 3.0);

        // Ensure interval out of range
        assert!(s.hit(&r, Interval { min: 4.0, max: 5.0 },).is_none());

        // Ensure that the ray does not hit the sphere
        let r = Ray {
            direction: Vec3::new_int(2, 0, 1),
            ..r
        };
        assert!(s
            .hit(
                &r,
                Interval {
                    min: 0.001,
                    max: INFINITY,
                },
            )
            .is_none());
    }
}
