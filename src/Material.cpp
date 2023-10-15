#include "Material.h"
#include "Color.h"
#include "Hittable.h"
#include "Ray.h"
#include "Vec3.h"

namespace material {
bool Lambertian::scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
                         color::Color &attenuation,
                         ray::Ray &scattered_r) const {
  auto scatter_direction =
      rec.against_unit_normal + vec::random_unit_vector_in_unit_sphere();
  if (scatter_direction.near_zero()) {
    // Catch ray cancelling out
    scatter_direction = rec.against_unit_normal;
  }
  attenuation = albedo;
  scattered_r = ray::Ray(rec.p, scatter_direction);
  return true;
}

bool Metal::scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
                    color::Color &attenuation, ray::Ray &scattered_r) const {
  auto scattered_direction =
      vec::reflect(r_in.get_direction(), rec.against_unit_normal);
  attenuation = albedo;
  scattered_r = ray::Ray(rec.p, scattered_direction);
  return true;
}

} // namespace material
