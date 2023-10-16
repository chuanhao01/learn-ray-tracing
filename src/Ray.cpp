#include "Ray.h"
#include "Color.h"
#include "Objects.h"
#include "Vec3.h"

namespace ray {
std::ostream &operator<<(std::ostream &cout, const Ray &r) {
  return cout << "Origin: " << r.get_origin()
              << ", Direction: " << r.get_direction();
}
} // namespace ray
