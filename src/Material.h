#ifndef MATERIAL_H
#define MATERIAL_H

#include "Color.h"
#include "Hittable.h"
#include "Ray.h"

namespace hittable {
class Hit_Record;
}

namespace material {
class Material {
public:
  virtual ~Material() = default;
  virtual bool scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
                       color::Color &attenuation,
                       ray::Ray &scattered_r) const = 0;
};

class Lambertian : public Material {
public:
  Lambertian(const color::Color &_albedo) : albedo(_albedo) {}

  bool scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
               color::Color &attenuation, ray::Ray &scattered_r) const override;

private:
  color::Color albedo;
};

/**
 * @brief
 *
 * @param _fuzzy_factor Factor of the size of the unit circle to generate
 * reflection rays around
 */
class Metal : public Material {
public:
  Metal(const color::Color &_albedo, double _fuzzy_factor)
      : albedo(_albedo), fuzzy_factor(_fuzzy_factor < 1 ? _fuzzy_factor : 1) {}

  bool scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
               color::Color &attenuation, ray::Ray &scattered_r) const override;

private:
  color::Color albedo;
  double fuzzy_factor;
};
} // namespace material

#endif
