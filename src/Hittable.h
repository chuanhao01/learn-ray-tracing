#ifndef HITTABLE_H
#define HITTABLE_H

#include "Interval.h"
#include "Ray.h"
#include "Vec3.h"

namespace hittable {
/**
 * @brief
 * The class to hold the information of a ray hitting an object
 */
class Hit_Record {
public:
  /**
   * @param against_unit_normal The unit vector of the normal against the
   * direction of the hit
   * @param p The point at which the ray hits the object
   * @param t The t value used from the ray to generate the hit
   * @param font_face If the ray hits is from the outside
   */
  vec::Vec3 against_unit_normal;
  vec::Point3 p;
  double t;
  bool front_face;

  void set_face_normal(const ray::Ray &r, const vec::Vec3 &outward_unit_normal);
};

class Hittable {
public:
  virtual ~Hittable() = default;

  virtual bool hit(const ray::Ray &r, interval::Interval valid_ray_t,
                   Hit_Record &rec) const = 0;
};
} // namespace hittable

#endif
