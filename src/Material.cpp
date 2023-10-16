#include "Material.h"
#include "Color.h"
#include "Hittable.h"
#include "RTWeekend.h"
#include "Ray.h"
#include "Vec3.h"

#include <cmath>

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

  double cos_theta =
      std::fmin(vec::dot(-unit_direction, rec.against_unit_normal), 1.0);
  double sin_theta = std::sqrt(1.0 - cos_theta * cos_theta);

  vec::Vec3 scattered_v;

  if (refraction_ratio * sin_theta > 1.0 ||
      reflectance(cos_theta, refraction_ratio) > random_double()) {
    // Need to reflection
    scattered_v = vec::reflect(unit_direction, rec.against_unit_normal);
  } else {
    // Can Refract
    scattered_v =
        vec::refract(unit_direction, rec.against_unit_normal, refraction_ratio);
  }
  scattered_r = ray::Ray(rec.p, scattered_v);
  return true;
}

double Dielectric::reflectance(double cosine, double refraction_ratio) {
  // Use Schlick's approximation for reflectance
  auto r0 = (1 - refraction_ratio) / (1 + refraction_ratio);
  r0 = r0 * r0;
  return r0 + (1 - r0) * std::pow(1 - cosine, 5);
}

} // namespace material
