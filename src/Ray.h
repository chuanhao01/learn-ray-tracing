#ifndef RAY_H
#define RAY_H

#include "Color.h"
#include "Objects.h"
#include "Vec3.h"

namespace ray {
class Ray {
public:
  Ray() {}

  Ray(const vec::Point3 &origin, const vec::Vec3 &direction)
      : orig(origin), dir(direction) {}

  vec::Point3 origin() const { return orig; }
  vec::Vec3 direction() const { return dir; }

  vec::Point3 at(double t) const { return orig + t * dir; }

private:
  vec::Point3 orig;
  vec::Vec3 dir;
};

double hit_sphere(const objects::Sphere &s, const Ray &r);
color::Color color_ray(const Ray &r);
} // namespace ray

#endif
