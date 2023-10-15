#include "RTWeekend.h"

#include <iostream>

int engine() {
  // World
  hittable_list::Hittable_List world;
  auto material_ground =
      std::make_shared<material::Lambertian>(color::Color(0.8, 0.8, 0.0));
  auto material_center =
      std::make_shared<material::Lambertian>(color::Color(0.7, 0.3, 0.3));
  auto material_left =
      std::make_shared<material::Metal>(color::Color(0.8, 0.8, 0.8), 0);
  auto material_right =
      std::make_shared<material::Metal>(color::Color(0.8, 0.6, 0.2), 0.9);
  auto material_dielectric = std::make_shared<material::Dielectric>(1.5);

  world.add(std::make_shared<objects::Sphere>(vec::Point3(0, 0, -1), 0.5,
                                              material_dielectric));
  world.add(std::make_shared<objects::Sphere>(vec::Point3(1, 0, -1), 0.5,
                                              material_right));
  world.add(std::make_shared<objects::Sphere>(vec::Point3(-1, 0, -1), 0.5,
                                              material_dielectric));
  world.add(std::make_shared<objects::Sphere>(vec::Point3(-5, 4, -10), 2,
                                              material_center));
  world.add(std::make_shared<objects::Sphere>(vec::Point3(0, -100.5, -1), 100,
                                              material_ground));

  // Camera
  camera::Camera cam;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 20;

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
