#include "Ray.h"
#include "Color.h"
#include "Objects.h"
#include "Vec3.h"

namespace ray {

color::Color color_ray(const Ray &r) {
  // objects::Sphere s1(vec::Point3(0, 0, -1), 0.5);
  // auto t1 = hit_sphere(s1, r);
  // if (t1 > 0.0) {
  //   vec::Vec3 N = vec::unit_vector(r.at(t1) - s1.center());
  //   return 0.5 * color::Color(N.x() + 1, N.y() + 1, N.z() + 1);
  // }

  auto unit_direction = vec::unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  return (1 - a) * color::Color(1.0, 1.0, 1.0) +
         a * color::Color(0.5, 0.7, 1.0);
}
} // namespace ray
