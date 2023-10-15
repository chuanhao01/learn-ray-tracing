#include "RTWeekend.h"

#include <iostream>

int engine() {
  // World
  hittable_list::Hittable_List world1, world2;
  auto material_ground =
      std::make_shared<material::Lambertian>(color::Color(0.8, 0.8, 0.0));
  auto material_center =
      std::make_shared<material::Lambertian>(color::Color(0.1, 0.2, 0.5));
  auto material_left =
      std::make_shared<material::Metal>(color::Color(0.8, 0.8, 0.8), 0);
  auto material_right =
      std::make_shared<material::Metal>(color::Color(0.8, 0.6, 0.2), 0.0);
  auto material_dielectric = std::make_shared<material::Dielectric>(1.5);

  world1.add(std::make_shared<objects::Sphere>(vec::Point3(0, 0, -1), 0.5,
                                               material_center));
  world1.add(std::make_shared<objects::Sphere>(vec::Point3(1, 0, -1), 0.5,
                                               material_right));
  world1.add(std::make_shared<objects::Sphere>(vec::Point3(-1, 0, -1), -0.4,
                                               material_dielectric));
  world1.add(std::make_shared<objects::Sphere>(vec::Point3(-1, 0, -1), 0.5,
                                               material_dielectric));
  // world1.add(std::make_shared<objects::Sphere>(vec::Point3(-5, 4, -10), 2,
  //                                             material_center));
  world1.add(std::make_shared<objects::Sphere>(vec::Point3(0, -100.5, -1), 100,
                                               material_ground));

  auto R = cos(PI / 4);

  auto mat_blue = std::make_shared<material::Lambertian>(color::Color(0, 0, 1));
  auto mat_red = std::make_shared<material::Metal>(color::Color(1, 0, 0), 0);
  world2.add(
      std::make_shared<objects::Sphere>(vec::Point3(-R, 0, -1), R, mat_blue));
  world2.add(
      std::make_shared<objects::Sphere>(vec::Point3(R, 0, -1), R, mat_red));

  // Camera
  camera::Camera cam;
  cam.image_width = 400;
  // cam.focal_length = 0.5;
  cam.samples_per_pixel = 40;
  cam.max_depth = 10;

  cam.render(world2);

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
