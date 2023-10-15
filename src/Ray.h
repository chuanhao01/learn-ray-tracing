#ifndef RAY_H
#define RAY_H

#include "Color.h"
#include "Vec3.h"

#include <iostream>

namespace ray {
/**
 * @brief
 * Represents a Ray in the form of
 * Ray = origin + t * direction
 *
 * @param origin Point of origin of the ray
 */
class Ray {
public:
  Ray() {}

  Ray(const vec::Point3 &_origin, const vec::Vec3 &_direction)
      : origin(_origin), direction(_direction) {}

  vec::Point3 get_origin() const { return origin; }
  vec::Vec3 get_direction() const { return direction; }

  vec::Point3 at(double t) const { return origin + t * direction; }

private:
  vec::Point3 origin;
  vec::Vec3 direction;
};

std::ostream &operator<<(std::ostream &cout, const Ray &r);

} // namespace ray

#endif
