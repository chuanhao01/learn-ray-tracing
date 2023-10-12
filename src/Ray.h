#ifndef RAY_H
#define RAY_H

#include "Vec3.h"

class ray {
public:
  ray() {}

  ray(const vec::Point3 &origin, const vec::Vec3 &direction)
      : orig(origin), dir(direction) {}

  vec::Point3 origin() const { return orig; }
  vec::Vec3 direction() const { return dir; }

  vec::Point3 at(double t) const { return orig + t * dir; }

private:
  vec::Point3 orig;
  vec::Vec3 dir;
};

#endif
