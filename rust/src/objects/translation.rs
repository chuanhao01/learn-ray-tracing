use std::{fmt::Display, sync::Arc};

use super::{HittableObject, Hittables};
use crate::{HitRecord, Hittable, Interval, Ray, Vec3, AABB};

/// Translation Object that takes a shallow copy of the original and translates it position by the offset Vector
pub struct Translation {
    instance: Arc<Hittables>,
    offset: Vec3,
    bbox: AABB,
}
impl Translation {
    pub fn new(instance: Arc<Hittables>, offset: Vec3) -> Self {
        Self {
            instance: instance.clone(),
            offset: offset.clone(),
            bbox: instance.bbox().translate(offset.clone()),
        }
    }
}
impl Hittable<HitRecord> for Translation {
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
impl HittableObject for Translation {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
impl Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Translation(offset: {}, {})", self.offset, self.instance)
    }
}

#[cfg(test)]
mod test {
    use crate::{Materials, ScatterMaterials, Sphere};

    use super::*;
    #[test]
    fn test_simple_translation_hit() {
        let original_object = Arc::new(Hittables::Sphere(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(Materials::ScatterMaterial(ScatterMaterials::None)),
        )));
    }
}
