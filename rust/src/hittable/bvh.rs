use rand::{thread_rng, Rng};
use std::{cmp::Ordering, sync::Arc};

use crate::{Interval, Ray};

use super::{HitRecord, Hittable};

mod aabb;
pub use aabb::AABB;

pub trait HittableWithBBox: Hittable {
    fn bbox(&self) -> &AABB;
}

/// Bounding volume hierarchy
pub struct BVH {
    left: Option<Box<BVH>>,
    right: Option<Box<BVH>>,
    bbox: AABB,
    hittable: Option<Arc<dyn HittableWithBBox>>,
}
impl BVH {
    pub fn from_hittables_list(hittable_list: Vec<Arc<dyn HittableWithBBox>>) -> Self {
        Self::new(&hittable_list[..], 0, hittable_list.len())
    }
    #[allow(clippy::clone_on_copy)]
    fn new(hittables: &[Arc<dyn HittableWithBBox>], start: usize, end: usize) -> Self {
        let mut rng = thread_rng();
        let axis = rng.gen_range(0_i64..3_i64);

        let hittable_comparer =
            |a: &Arc<dyn HittableWithBBox>, b: &Arc<dyn HittableWithBBox>| -> Ordering {
                if a.bbox().axis(axis).min < b.bbox().axis(axis).min {
                    Ordering::Less
                } else if a.bbox().axis(axis).min > b.bbox().axis(axis).min {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            };
        let list_size = end - start;
        if list_size == 0 {
            BVH {
                left: None,
                right: None,
                hittable: None,
                bbox: AABB::default(),
            }
        } else if list_size == 1 {
            BVH {
                left: None,
                right: None,
                hittable: Some(hittables[start].clone()),
                bbox: hittables[start].bbox().clone(),
            }
        } else if list_size == 2 {
            BVH {
                left: Some(Box::new(BVH {
                    left: None,
                    right: None,
                    hittable: Some(hittables[start].clone()),
                    bbox: hittables[start].bbox().clone(),
                })),
                right: Some(Box::new(BVH {
                    left: None,
                    right: None,
                    hittable: Some(hittables[end - 1].clone()),
                    bbox: hittables[end - 1].bbox().clone(),
                })),
                hittable: None,
                bbox: AABB::from_aabb(hittables[start].bbox(), hittables[end - 1].bbox()),
            }
        } else {
            let mut hittables = hittables.to_vec();
            // Small bug, I used to sort the whole vector
            // hittables.sort_by(hittable_comparer);
            // Since the vector is sorted everytime by a random axis
            // Example with 4
            // The left side places 0, 1 from 0, 1, 2, 3
            // Then the right side sorts 2, 3, 0, 1 With the chosen
            // hittables.sort_by(hittable_comparer);
            hittables[start..end].sort_by(hittable_comparer);
            let mid = start + list_size / 2;
            let left = Self::new(&hittables, start, mid);
            let right = Self::new(&hittables, mid, end);
            let bbox = AABB::from_aabb(&left.bbox, &right.bbox);

            BVH {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
                hittable: None,
                bbox,
            }
        }
    }
}
impl Hittable for BVH {
    fn hit(&self, _ray: &Ray, valid_t_interval: Interval) -> Option<HitRecord> {
        // Check if we don't hit the bbox of the BVH (Nicer rust code)
        self.bbox.hit(_ray, valid_t_interval)?;

        // Deal with base case of edge leaf nodes (Just the hittables)
        // Exit early to prevent computation
        if let (None, None) = (&self.left, &self.right) {
            match &self.hittable {
                Some(hittable) => return hittable.hit(_ray, valid_t_interval),
                // Used to be an error, now a default for no ndoes in the BVH
                // So it should hit nothing
                None => return None,
            };
        }

        match (&self.left, &self.right) {
            // BVH nodes which are not edges
            (Some(left), Some(right)) => {
                let mut valid_t_interval = valid_t_interval;
                if let Some(left_hit) = left.hit(_ray, valid_t_interval) {
                    valid_t_interval.max = left_hit.t;
                    if let Some(right_hit) = right.hit(_ray, valid_t_interval) {
                        Some(right_hit)
                    } else {
                        Some(left_hit)
                    }
                } else {
                    right.hit(_ray, valid_t_interval)
                }
            }
            // If there is a single Node or we reach (None, None) again
            // Should never reach here
            _ => {
                panic!("Trying to match left or right, but either no left or right or both")
            }
        }
    }
}
impl HittableWithBBox for BVH {
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

#[cfg(test)]
mod test {
    use crate::{materials::test::TestScatterable, Materials, Vec3};

    use super::*;

    #[derive(Clone)]
    struct TestHittable {
        v: Vec3,
        bbox: AABB,
    }
    impl Hittable for TestHittable {
        fn hit(&self, _ray: &Ray, _valid_t_interval: Interval) -> Option<HitRecord> {
            if _ray.direction == self.v {
                Some(HitRecord {
                    p: self.v.clone(),
                    t: 1.0,
                    material: Materials::ScatterMaterial(Arc::new(TestScatterable {})),
                    against_normal_unit: -self.v.clone(),
                    front_face: false,
                })
            } else {
                None
            }
        }
    }
    impl HittableWithBBox for TestHittable {
        fn bbox(&self) -> &AABB {
            &self.bbox
        }
    }

    #[test]
    fn test_bvh_hits_correct_test_hittable() {
        // Idea behind this feature
        // For the BVH, based on our test implementation,
        // It should hit the correct Hittable and return the HitRecord we implemented
        // This should happen regardless of which item it is in the list given or in the BVH
        fn check_bvh_hit_result(
            input_hittables: Vec<Arc<dyn HittableWithBBox>>,
            ouptut_hit_record: HitRecord,
        ) {
            let bvh = BVH::from_hittables_list(input_hittables);
            let hit_result = bvh.hit(
                &Ray {
                    origin: Vec3::new(0.0, 0.0, 0.0),
                    direction: Vec3::new(0.5, 0.5, 0.5),
                },
                Interval {
                    min: 0.0,
                    max: 10.0,
                },
            );
            assert!(hit_result.is_some());
            let hit_result = hit_result.unwrap();
            assert_eq!(hit_result.t, ouptut_hit_record.t);
            assert_eq!(hit_result.p, ouptut_hit_record.p);
            assert_eq!(
                hit_result.against_normal_unit,
                ouptut_hit_record.against_normal_unit
            );
            assert_eq!(hit_result.front_face, ouptut_hit_record.front_face);
        }

        let hit_test_hittable = Arc::new(TestHittable {
            v: Vec3::new(0.5, 0.5, 0.5),
            bbox: AABB::from_points(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        });
        let missed_test_hittable = Arc::new(TestHittable {
            v: Vec3::new(0.5, 0.5, 0.5),
            bbox: AABB::from_points(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(-2.0, -2.0, -2.0)),
        });

        check_bvh_hit_result(
            vec![
                hit_test_hittable.clone(),
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
            ],
            HitRecord {
                p: Vec3::new(0.5, 0.5, 0.5),
                t: 1.0,
                material: Materials::ScatterMaterial(Arc::new(TestScatterable {})),
                against_normal_unit: Vec3::new(-0.5, -0.5, -0.5),
                front_face: false,
            },
        );
        check_bvh_hit_result(
            vec![
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
                hit_test_hittable.clone(),
            ],
            HitRecord {
                p: Vec3::new(0.5, 0.5, 0.5),
                t: 1.0,
                material: Materials::ScatterMaterial(Arc::new(TestScatterable {})),
                against_normal_unit: Vec3::new(-0.5, -0.5, -0.5),
                front_face: false,
            },
        );
        check_bvh_hit_result(
            vec![
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
                hit_test_hittable.clone(),
                missed_test_hittable.clone(),
                missed_test_hittable.clone(),
            ],
            HitRecord {
                p: Vec3::new(0.5, 0.5, 0.5),
                t: 1.0,
                material: Materials::ScatterMaterial(Arc::new(TestScatterable {})),
                against_normal_unit: Vec3::new(-0.5, -0.5, -0.5),
                front_face: false,
            },
        );
    }

    #[test]
    fn test_bvh_hits_misses_test_hittable() {
        // Idea behind this feature
        // For the BVH, based on our test implementation,
        // It should miss all the hittables correctly, based on AABB

        let missed_test_hittable = Arc::new(TestHittable {
            v: Vec3::new(0.5, 0.5, 0.5),
            bbox: AABB::from_points(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(-2.0, -2.0, -2.0)),
        });

        fn check_bvh_misses(input_hittables: Vec<Arc<dyn HittableWithBBox>>) {
            let bvh = BVH::from_hittables_list(input_hittables);
            let hit_result = bvh.hit(
                &Ray {
                    origin: Vec3::new(0.0, 0.0, 0.0),
                    direction: Vec3::new(0.5, 0.5, 0.5),
                },
                Interval {
                    min: 0.0,
                    max: 10.0,
                },
            );
            assert!(hit_result.is_none());
        }
        check_bvh_misses(Vec::new());
        check_bvh_misses(vec![missed_test_hittable.clone()]);
        check_bvh_misses(vec![
            missed_test_hittable.clone(),
            missed_test_hittable.clone(),
        ]);
        check_bvh_misses(vec![
            missed_test_hittable.clone(),
            missed_test_hittable.clone(),
            missed_test_hittable.clone(),
        ]);
    }
}
