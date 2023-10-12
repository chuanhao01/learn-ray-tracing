#include "Ray.h"
#include "Color.h"
#include "Objects.h"
#include "Vec3.h"

#include <cmath>

namespace ray {
/**
 * @brief
 * Checks if the ray will intersect with the given sphere at all
 * Using the Formula of the sphere, and calculating based on the determinant of
 * solving for t
 *
 * @param s Sphere Object
 * @param r Ray to check
 * @return true
 * @return false
 */
double hit_sphere(const objects::Sphere &s, const Ray &r) {
  auto a_minus_c = r.origin() - s.center();

  auto a = vec::dot(r.direction(), r.direction());
  auto b = 2 * vec::dot(a_minus_c, r.direction());
  auto c = vec::dot(a_minus_c, a_minus_c) - s.radius() * s.radius();
  auto discriminant = b * b - 4 * a * c;
  if (discriminant < 0) {
    return -1.0;
  }
  return (-b - std::sqrt(discriminant)) / (2.0 * a);
}

color::Color color_ray(const Ray &r) {
  objects::Sphere s1(vec::Point3(0, 0, -1), 0.5);
  auto t1 = hit_sphere(s1, r);
  if (t1 > 0.0) {
    vec::Vec3 N = vec::unit_vector(r.at(t1) - vec::Vec3(0, 0, -1));
    return 0.5 * color::Color(N.x() + 1, N.y() + 1, N.z() + 1);
  }

  auto unit_direction = vec::unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  return (1 - a) * color::Color(1.0, 1.0, 1.0) +
         a * color::Color(0.5, 0.7, 1.0);
}
} // namespace ray
