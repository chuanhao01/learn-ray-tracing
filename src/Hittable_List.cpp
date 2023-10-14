#include "Hittable_List.h"
#include "Hittable.h"

#include <algorithm>

namespace hittable_list {
bool Hittable_List::hit(const ray::Ray &r, double ray_t_min, double ray_t_max,
                        hittable::Hit_Record &rec) const {
  hittable::Hit_Record temp_rec;
  bool hit_anything = false;
  auto closest_so_far = ray_t_max;
  for (const auto &object : objects) {
    if (object->hit(r, ray_t_min, closest_so_far, temp_rec)) {
      hit_anything = true;
      closest_so_far = temp_rec.t;
      rec = temp_rec;
    }
  }
  return hit_anything;
}
} // namespace hittable_list
