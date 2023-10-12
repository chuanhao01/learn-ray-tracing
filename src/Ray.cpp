#include "Ray.h"
#include "Color.h"
#include "Vec3.h"

namespace ray {
color::Color color_ray(const Ray &r) {
  auto unit_direction = vec::unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.length() + 1.0);
  return (1 - a) * color::Color(1.0, 1.0, 1.0) +
         a * color::Color(0.5, 0.7, 1.0);
}
} // namespace ray
