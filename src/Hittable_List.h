#ifndef HITTABLE_LIST_H
#define HITTABLE_LIST_H

#include "Hittable.h"
#include "Ray.h"

#include <memory>
#include <vector>

namespace hittable_list {
class Hittable_List : public hittable::Hittable {
public:
  std::vector<std::shared_ptr<hittable::Hittable>> objects;

  Hittable_List() {}
  Hittable_List(std::shared_ptr<hittable::Hittable> object) { add(object); }

  void add(std::shared_ptr<hittable::Hittable> object) {
    objects.push_back(object);
  }
  void clear() { objects.clear(); }

  bool hit(const ray::Ray &r, interval::Interval valid_ray_t,
           hittable::Hit_Record &rec) const override;
};
} // namespace hittable_list

#endif
