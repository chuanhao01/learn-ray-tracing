#ifndef OBJECT_H
#define OBJECT_H

#include "Hittable.h"
#include "Ray.h"
#include "Vec3.h"

namespace objects {
class Sphere : public hittable::Hittable {
public:
  Sphere() {}

  Sphere(const vec::Point3 &_center, double _radius)
      : center(_center), radius(_radius) {}

  bool hit(const ray::Ray &r, double ray_t_min, double ray_t_max,
           hittable::Hit_Record &rec) const;

private:
  double radius;
  vec::Point3 center;
};
} // namespace objects

#endif
