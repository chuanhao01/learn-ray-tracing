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
  Lambertian(const color::Color &a) : albedo(a) {}

  bool scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
               color::Color &attenuation, ray::Ray &scattered_r) const override;

private:
  color::Color albedo;
};

class Metal : public Material {
public:
  Metal(const color::Color &a) : albedo(a) {}

  bool scatter(const ray::Ray &r_in, const hittable::Hit_Record &rec,
               color::Color &attenuation, ray::Ray &scattered_r) const override;

private:
  color::Color albedo;
};
} // namespace material

#endif
