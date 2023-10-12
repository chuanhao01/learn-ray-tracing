#ifndef OBJECT_H
#define OBJECT_H

#include "Vec3.h"

namespace objects {
class Sphere {
public:
  Sphere() {}

  Sphere(const vec::Point3 &center, double radius) : cen(center), rad(radius) {}

  double radius() { return rad; }
  double radius() const { return rad; }
  vec::Point3 center() const { return cen; }

private:
  double rad;
  vec::Point3 cen;
};
} // namespace objects

#endif
