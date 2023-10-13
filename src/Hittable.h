#ifndef HITTABLE_H
#define HITTABLE_H

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
   * @param normal The normal vector of the hit
   * @param p The point at which the ray hits the object
   * @param t The t value used from the ray to generate the hit
   */
  vec::Vec3 normal;
  vec::Point3 p;
  double t;
};

class Hittable {
public:
  virtual ~Hittable() = default;

  virtual bool hit(const ray::Ray &r, double ray_t_min, double ray_t_max,
                   Hit_Record &rec) const = 0;
};
} // namespace hittable

#endif
