use std::{f64::INFINITY, fmt::Display, sync::Arc};

use super::{HittableObject, Hittables, Transformation};
use crate::{
    helper::from_fdegree_to_fradian, HitRecord, Hittable, Interval, Ray, Vec3, Vec3Axis, AABB,
};

struct Rotation {
    instance: Arc<Hittables>,
    rotation_axis: Vec3Axis,
    /// Angle of Rotation around the rotation_axis Degress(360)
    deg_angle: f64,
    bbox: AABB,
}
impl Rotation {
    /// Creates a new Rotation Hittables Object, given an axis and degrees (360) of rotation
    fn new(instance: Arc<Hittables>, rotation_axis: Vec3Axis, deg_angle: f64) -> Self {
        let aabb = instance.bbox();
        let mut bottom_left_min_aabb_point = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut top_right_max_aabb_point = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        // Looping through all 8 points of the aabb
        for x in [aabb.x.min, aabb.x.max] {
            for y in [aabb.y.min, aabb.y.max] {
                for z in [aabb.z.min, aabb.z.max] {
                    let object_point = Vec3::new(x, y, z);
                    let world_point = object_point.rotate_about_axis(&rotation_axis, deg_angle);
                    bottom_left_min_aabb_point =
                        bottom_left_min_aabb_point.retain_min(&world_point);
                    top_right_max_aabb_point = top_right_max_aabb_point.retain_max(&world_point);
                }
            }
        }

        Self {
            instance,
            rotation_axis,
            deg_angle,
            bbox: AABB::from_points(&bottom_left_min_aabb_point, &top_right_max_aabb_point),
        }
    }
}
impl Hittable<HitRecord> for Rotation {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        let new_ray = Ray{

        }
        let new_origin = _ray
            .origin
            .rotate_about_axis(&self.rotation_axis, -self.deg_angle);
        let new_direction = _ray
            .direction
            .rotate_about_axis(&self.rotation_axis, -self.deg_angle);
        None
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rotation(rotation_axis: {}, rad_angle: {}, instance: {})",
            self.rotation_axis, self.deg_angle, self.instance
        )
    }
}
