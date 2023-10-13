#include "Objects.h"
#include "Hittable.h"
#include "Ray.h"
#include "Vec3.h"

#include <cmath>

namespace objects {

bool Sphere::hit(const ray::Ray &r, double ray_t_min, double ray_t_max,
                 hittable::Hit_Record &rec) const {

  auto a_minus_c = r.origin() - center;

  auto a = r.direction().length_squared();
  auto b = vec::dot(a_minus_c, r.direction());
  auto c = a_minus_c.length_squared() - radius * radius;
  auto discriminant = b * b - a * c;
  if (discriminant < 0) {
    return false;
  }

  // Taking the closer point in which the ray intersects the sphere
  auto sqrt_discriminant = std::sqrt(discriminant);
  auto root = (-b - sqrt_discriminant) / a;
  if (root <= ray_t_min || root >= ray_t_max) {
    // Given root is not within range, so checking other root
    root = (-b + sqrt_discriminant) / a;
    if (root <= ray_t_min || root >= ray_t_max) {
      return false;
    }
  }

  // Creating the hit_record with the passed by reference rec
  rec.t = root;
  rec.p = r.at(root);
  rec.normal = (rec.p - center) / radius;
  return true;
}
} // namespace objects
