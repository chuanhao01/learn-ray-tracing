#include "RTWeekend.h"

double degrees_to_radians(double degrees) { return degrees / 180.0 * PI; }

color::Color color_ray(const ray::Ray &r,
                       const hittable_list::Hittable_List &world) {
  hittable::Hit_Record rec;
  if (world.hit(r, interval::Interval(0, infinity), rec)) {
    // Visualize the unit normal as rgb
    return 0.5 * (rec.against_unit_normal + vec::Vec3(1, 1, 1));
  }
  // If the ray does not hit anything, visualize as y value from white to blue
  auto unit_direction = vec::unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  return (1 - a) * color::Color(1.0, 1.0, 1.0) +
         a * color::Color(0.5, 0.7, 1.0);
}
