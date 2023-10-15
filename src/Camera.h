#ifndef CAMERA_H
#define CAMERA_H

#include "Color.h"
#include "Hittable_List.h"
#include "Ray.h"
#include "Vec3.h"

namespace camera {
class Camera {
public:
  // Variables user can set
  double aspect_ratio = 16.0 / 9.0;
  int image_width = 400;

  double focal_length = 1.0;
  double viewport_height = 2.0;

  int samples_per_pixel = 10;
  int max_depth = 10;

  vec::Vec3 center = vec::Vec3(0, 0, 0);

  void render(const hittable_list::Hittable_List &world);

private:
  int image_height;
  double viewport_width;

  vec::Vec3 pixel_delta_u;
  vec::Vec3 pixel_delta_v;

  vec::Point3 pixel00_loc;

  void init();
  ray::Ray get_ray(double j, double i);

  vec::Point3 pixel_sample_square();

  color::Color color_ray(const ray::Ray &r, int max_depth,
                         const hittable_list::Hittable_List &world);
};
} // namespace camera

#endif
