#include "RTWeekend.h"

#include <iostream>

int engine() {
  // World
  hittable_list::Hittable_List world;
  world.add(std::make_shared<objects::Sphere>(vec::Point3(0, 0, -1), 0.5));
  world.add(std::make_shared<objects::Sphere>(vec::Point3(-5, 2, -10), 2));
  world.add(std::make_shared<objects::Sphere>(vec::Point3(0, -100.5, -1), 100));

  // Camera
  camera::Camera cam;
  cam.image_width = 400;
  cam.samples_per_pixel = 100;
  cam.max_depth = 50;

  cam.render(world);

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
