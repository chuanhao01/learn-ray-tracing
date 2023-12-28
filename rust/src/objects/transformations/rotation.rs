use std::f64::INFINITY;

use crate::{helper::from_fdegree_to_fradian, Vec3, Vec3Axis, AABB};

use super::Transformation;

pub struct Rotation {
    axis: Vec3Axis,
    deg_angle: f64,
}
