#include "Camera.h"
#include "Color.h"
#include "Hittable_List.h"
#include "RTWeekend.h"
#include "Ray.h"
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
      color::Color pixel_color(0, 0, 0);
      for (int sample = 0; sample < samples_per_pixel; sample++) {
        ray::Ray r = get_ray(j, i);
        pixel_color += color_ray(r, max_depth, world);
      }

      color::write_color(std::cout, pixel_color, samples_per_pixel);
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

/**
 * @brief
 * Returns a random ray for a given pixel at (y, x) => (j, i)
 * @param j y position of the pixel
 * @param i x position of the pixel
 * @return A sampled ray within the square of the pixel
 */
ray::Ray Camera::get_ray(double j, double i) {

  auto pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
  auto pixel_sample = pixel_center + pixel_sample_square();

  auto ray_direction = pixel_sample - center;
  return ray::Ray(center, ray_direction);
}

vec::Point3 Camera::pixel_sample_square() {
  auto x = random_double() - 0.5;
  auto y = random_double() - 0.5;
  return x * pixel_delta_u + y * pixel_delta_v;
}

color::Color Camera::color_ray(const ray::Ray &r, int max_depth,
                               const hittable_list::Hittable_List &world) {
  // If reached max ray depth, return black (no light)
  if (max_depth <= 0) {
    return color::Color(0, 0, 0);
  }

  hittable::Hit_Record rec;
  /**
   * Subtle rounding error, t interval to be larger than treshold as light could
   * bounch right below the surface
   */
  if (world.hit(r, interval::Interval(0.001, infinity), rec)) {
    // Using random diffuse off hemisphere
    // vec::Vec3 diffuse_direction =
    //     vec::random_unit_vector_on_hemisphere(rec.against_unit_normal);

    // Using Lambertain Reflection
    // vec::Vec3 diffuse_direction =
    //     rec.against_unit_normal + vec::random_unit_vector_in_unit_sphere();

    // Using Materials
    ray::Ray scattered_r;
    color::Color attenuation;
    if (rec.mat->scatter(r, rec, attenuation, scattered_r)) {
      return attenuation * color_ray(scattered_r, max_depth - 1, world);
    }
    // If you hit something without a material
    return color::Color(0, 0, 0);
  }
  // If the ray does not hit anything, visualize as y value from white to blue
  auto unit_direction = vec::unit_vector(r.get_direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  return (1 - a) * color::Color(1.0, 1.0, 1.0) +
         a * color::Color(0.5, 0.7, 1.0);
}
} // namespace camera
