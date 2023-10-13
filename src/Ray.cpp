#include "Ray.h"
#include "Color.h"
#include "Objects.h"
#include "Vec3.h"

#include <cmath>

namespace ray {

/**
 * @brief
 * Calculate if a ray hits the sphere using the formula
 * returns t value for the nearer ray that hits the sphere
 * If the ray does not hit the sphere -1
 * @param s Sphere Object
 * @param r Ray to check
 * @return double
 */
double hit_sphere(const objects::Sphere &s, const Ray &r) {
  auto a_minus_c = r.origin() - s.center();

  auto a = r.direction().length_squared();
  auto b = vec::dot(a_minus_c, r.direction());
  auto c = a_minus_c.length_squared() - s.radius() * s.radius();
  auto discriminant = b * b - a * c;
  if (discriminant < 0) {
    return -1.0;
  }
  // Taking the closer point in which the ray intersects the sphere
  return (-b - std::sqrt(discriminant)) / a;
}

color::Color color_ray(const Ray &r) {
  objects::Sphere s1(vec::Point3(0, 0, -1), 0.5);
  auto t1 = hit_sphere(s1, r);
  if (t1 > 0.0) {
    vec::Vec3 N = vec::unit_vector(r.at(t1) - s1.center());
    return 0.5 * color::Color(N.x() + 1, N.y() + 1, N.z() + 1);
  }

  auto unit_direction = vec::unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  return (1 - a) * color::Color(1.0, 1.0, 1.0) +
         a * color::Color(0.5, 0.7, 1.0);
}
} // namespace ray
