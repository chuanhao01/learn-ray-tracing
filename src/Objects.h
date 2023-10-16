#ifndef OBJECT_H
#define OBJECT_H

#include "Hittable.h"
#include "Interval.h"
#include "Material.h"
#include "Ray.h"
#include "Vec3.h"

#include <memory>

namespace objects {
class Sphere : public hittable::Hittable {
public:
  Sphere() {}

  Sphere(const vec::Point3 &_center, double _radius,
         std::shared_ptr<material::Material> _material)
      : center(_center), radius(_radius), material(_material) {}

  bool hit(const ray::Ray &r, interval::Interval valid_ray_t,
           hittable::Hit_Record &rec) const;

private:
  double radius;
  vec::Point3 center;
  std::shared_ptr<material::Material> material;
};
} // namespace objects

#endif
