#include "Camera.h"
#include "Color.h"
#include "Hittable_List.h"
#include "Vec3.h"

#include <iostream>

namespace camera {
void Camera::render(const hittable_list::Hittable_List &world) {
  init();

  // Doing it from top left
  std::cout << "P3"
            << "\n";
  std::cout << image_width << " " << image_height << "\n";
  std::cout << "255"
            << "\n";

  // (y, x), (j, i)
  for (int j = 0; j < image_height; j++) {
    // Per column
    std::cerr << "\r"
              << "Scanlines remaining: " << image_height - j - 1 << std::flush;
    for (int i = 0; i < image_width; i++) {
      auto pixel_center =
          pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
      auto ray_direction = pixel_center - center;
      ray::Ray r(center, ray_direction);

      auto pixel_color = color_ray(r, world);
      color::write_color(std::cout, pixel_color);
      std::cout << "\n";
    }
  }
  std::cerr << "\n"
            << "Done!"
            << "\n";
}

void Camera::init() {
  image_height = static_cast<int>(image_width / aspect_ratio);

  viewport_width =
      viewport_height * (static_cast<double>(image_width) / image_height);

  auto delta_u = vec::Vec3(viewport_width, 0, 0);
  auto delta_v = vec::Vec3(0, -viewport_height, 0);
  pixel_delta_u = delta_u / image_width;
  pixel_delta_v = delta_v / image_height;

  auto upper_left_corner =
      center - vec::Vec3(0, 0, focal_length) - 0.5 * (delta_u + delta_v);
  pixel00_loc = upper_left_corner + 0.5 * (pixel_delta_u + pixel_delta_v);
}

color::Color Camera::color_ray(const ray::Ray &r,
                               const hittable_list::Hittable_List &world) {
  hittable::Hit_Record rec;
  if (world.hit(r, interval::Interval(0, infinity), rec)) {
    // Visualize the unit normal as rgb
    return 0.5 * (rec.against_unit_normal + vec::Vec3(1, 1, 1));
  }
  // If the ray does not hit anything, visualize as y value from white to blue
  auto unit_direction = vec::unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  return (1 - a) * color::Color(1.0, 1.0, 1.0) +
         a * color::Color(0.5, 0.7, 1.0);
}
} // namespace camera
