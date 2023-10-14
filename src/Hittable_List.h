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

  bool hit(const ray::Ray &r, double ray_t_min, double ray_t_max,
           hittable::Hit_Record &rec);
};
} // namespace hittable_list

#endif
