#include "Hittable_List.h"
#include "Hittable.h"
#include "Interval.h"

#include <algorithm>

namespace hittable_list {
bool Hittable_List::hit(const ray::Ray &r, interval::Interval valid_ray_t,
                        hittable::Hit_Record &rec) const {
  hittable::Hit_Record temp_rec;
  bool hit_anything = false;
  for (const auto &object : objects) {
    if (object->hit(r, valid_ray_t, temp_rec)) {
      hit_anything = true;
      valid_ray_t.max = temp_rec.t;
      rec = temp_rec;
    }
  }
  return hit_anything;
}
} // namespace hittable_list
