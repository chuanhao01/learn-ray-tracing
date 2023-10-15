#include "Objects.h"
#include "Hittable.h"
#include "Interval.h"
#include "Ray.h"
#include "Vec3.h"

#include <cmath>

namespace objects {

bool Sphere::hit(const ray::Ray &r, interval::Interval valid_ray_t,
                 hittable::Hit_Record &rec) const {

  vec::Vec3 a_minus_c = r.get_origin() - center;

  auto a = r.get_direction().length_squared();
  auto b = vec::dot(a_minus_c, r.get_direction());
  auto c = a_minus_c.length_squared() - radius * radius;
  auto discriminant = b * b - a * c;
  if (discriminant < 0) {
    return false;
  }

  // Taking the closer point in which the ray intersects the sphere
  auto sqrt_discriminant = std::sqrt(discriminant);
  auto root = (-b - sqrt_discriminant) / a;
  if (!valid_ray_t.surrounds(root)) {
    // Given root is not within range, so checking other root
    root = (-b + sqrt_discriminant) / a;
    if (!valid_ray_t.surrounds(root)) {
      return false;
    }
  }

  // Creating the hit_record with the passed by reference rec
  rec.t = root;
  rec.p = r.at(rec.t);
  vec::Vec3 outward_normal = rec.p - center;
  rec.set_face_normal(r, outward_normal / radius);
  return true;
}
} // namespace objects
