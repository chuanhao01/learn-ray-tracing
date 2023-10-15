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
  scattered_r = ray::Ray(
      rec.p, scattered_direction +
                 fuzzy_factor * vec::random_unit_vector_in_unit_sphere());
  return (dot(rec.against_unit_normal, scattered_r.get_direction()) > 0.0);
}

bool Dielectric::scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
                         color::Color &attenuation,
                         ray::Ray &scattered_r) const {
  attenuation = color::Color(1.0, 1.0, 1.0);
  double refraction_ratio =
      rec.front_face ? (1.0 / index_of_refraction) : index_of_refraction;

  vec::Vec3 unit_direction = vec::unit_vector(r_in.get_direction());
  vec::Vec3 refracted =
      vec::refract(unit_direction, rec.against_unit_normal, refraction_ratio);

  scattered_r = ray::Ray(rec.p, refracted);
  return true;
}

} // namespace material
