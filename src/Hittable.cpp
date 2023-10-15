#include "Hittable.h"
#include "Vec3.h"

namespace hittable {
/**
 * @brief
 * Given the hit ray and outward unit normal vector,
 * Calculates and sets the front_face and against_unit_normal
 * @param r Ray that hits the object
 * @param outward_unit_normal Unit Vector of the normal travelling outwards from
 * the object
 */
void Hit_Record::set_face_normal(const ray::Ray &r,
                                 const vec::Vec3 &outward_unit_normal) {
  front_face = vec::dot(r.get_direction(), outward_unit_normal);
  against_unit_normal = front_face ? outward_unit_normal : -outward_unit_normal;
}
} // namespace hittable
