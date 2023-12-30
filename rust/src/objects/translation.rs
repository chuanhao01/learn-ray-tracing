use std::{fmt::Display, sync::Arc};

use crate::{HitRecord, Hittable, HittableWithBBox, Interval, Ray, Vec3, AABB};

/// Translation Object that takes a shallow copy of the original and translates it position by the offset Vector
pub struct Translation {
    instance: Arc<dyn HittableWithBBox>,
    offset: Vec3,
    bbox: AABB,
}
impl Translation {
    pub fn new(instance: Arc<dyn HittableWithBBox>, offset: Vec3) -> Self {
        Self {
            instance: instance.clone(),
            offset: offset.clone(),
            bbox: instance.bbox().translate(offset.clone()),
        }
    }
}
impl Hittable for Translation {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        // We are checking the ray that hits where the instance object is (has the offset already applied)
        // Therefore we will move the ray back to where the original object is, then move the hit back by the offset
        let translated_ray = Ray {
            origin: _ray.origin.clone() - self.offset.clone(),
            .._ray.clone()
        };
        match self.instance.hit(&translated_ray, valid_t_interval) {
            Some(mut hit_record) => {
                hit_record.p += self.offset.clone();
                Some(hit_record)
            }
            None => None,
        }
    }
}
impl HittableWithBBox for Translation {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
// impl Display for Translation {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Translation(offset: {}, {})", self.offset, self.instance)
//     }
// }
