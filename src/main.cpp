#include "Color.h"
#include "Ray.h"
#include "Vec3.h"

#include <iostream>

int engine() {
  // Image
  auto ASPECT_RATIO = 16.0 / 9.0;

  int IMAGE_WIDTH = 400;
  int IMAGE_HEIGHT = static_cast<int>(IMAGE_WIDTH / ASPECT_RATIO);
  IMAGE_HEIGHT = (IMAGE_HEIGHT < 1) ? 1 : IMAGE_HEIGHT;

  // Camera
  auto FOCAL_LENGTH = 1.0;
  auto VIEWPORT_HEIGHT = 2.0;
  auto VIEWPORT_WIDTH =
      VIEWPORT_HEIGHT * (static_cast<double>(IMAGE_WIDTH) / IMAGE_HEIGHT);
  auto camera_center = vec::Point3(0, 0, 0);

  auto viewport_u = vec::Vec3(VIEWPORT_WIDTH, 0, 0);
  auto viewport_v = vec::Vec3(0, -VIEWPORT_HEIGHT, 0);

  auto pixel_delta_u = viewport_u / IMAGE_WIDTH;
  auto pixel_delta_v = viewport_v / IMAGE_HEIGHT;

  auto viewport_upper_left = camera_center - vec::Vec3(0, 0, FOCAL_LENGTH) -
                             viewport_u / 2 - viewport_v / 2;
  auto pixel00_loc =
      viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

  // Render
  // Doing it from top left
  std::cout << "P3"
            << "\n";
  std::cout << IMAGE_WIDTH << " " << IMAGE_HEIGHT << "\n";
  std::cout << "255"
            << "\n";

  // (y, x), (j, i)
  for (int j = 0; j < IMAGE_HEIGHT; j++) {
    // Per column
    std::cerr << "\r"
              << "Scanlines remaining: " << IMAGE_HEIGHT - j - 1 << std::flush;
    for (int i = 0; i < IMAGE_WIDTH; i++) {
      auto pixel_center =
          pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
      auto ray_direction = pixel_center - camera_center;
      ray::Ray r(camera_center, ray_direction);

      auto pixel_color = ray::color_ray(r);
      color::write_color(std::cout, pixel_color);
      std::cout << "\n";
    }
  }
  std::cerr << "\n"
            << "Done!"
            << "\n";

  return 0;
}

int testMath() {
  const int B = 255, BB = 256;
  for (int i = 0; i < BB; i++) {
    auto a = double(i % (B + 1)) / B;
    auto aa = double(i) / B;
    std::cout << i << "\n";
    std::cout << a << "\n";
    std::cout << aa << "\n";
    std::cout << "\n";
  }
  std::cout << "Done!"
            << "\n";
  return 0;
}

int main() { engine(); }
