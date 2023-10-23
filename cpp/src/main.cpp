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
  auto mat_red = std::make_shared<material::Lambertian>(color::Color(1, 0, 0));
  world2.add(
      std::make_shared<objects::Sphere>(vec::Point3(-R, 0, -1), R, mat_blue));
  world2.add(
      std::make_shared<objects::Sphere>(vec::Point3(R, 0, -1), R, mat_red));

  // Final render world
  hittable_list::Hittable_List final_world;
  auto ground_material =
      std::make_shared<material::Lambertian>(color::Color(0.5, 0.5, 0.5));
  final_world.add(std::make_shared<objects::Sphere>(vec::Point3(0, -1000, 0),
                                                    1000, ground_material));

  for (int a = -11; a < 11; a++) {
    for (int b = -11; b < 11; b++) {
      auto choose_mat = random_double();
      vec::Point3 center(a + 0.9 * random_double(), 0.2,
                         b + 0.9 * random_double());

      if ((center - vec::Point3(4, 0.2, 0)).length() > 0.9) {
        std::shared_ptr<material::Material> sphere_material;

        if (choose_mat < 0.8) {
          // diffuse
          auto albedo = color::Color::random() * color::Color::random();
          sphere_material = std::make_shared<material::Lambertian>(albedo);
          final_world.add(
              std::make_shared<objects::Sphere>(center, 0.2, sphere_material));
        } else if (choose_mat < 0.95) {
          // metal
          auto albedo = color::Color::random(0.5, 1);
          auto fuzz = random_double(0, 0.5);
          sphere_material = std::make_shared<material::Metal>(albedo, fuzz);
          final_world.add(
              std::make_shared<objects::Sphere>(center, 0.2, sphere_material));
        } else {
          // glass
          sphere_material = std::make_shared<material::Dielectric>(1.5);
          final_world.add(
              std::make_shared<objects::Sphere>(center, 0.2, sphere_material));
        }
      }
    }
  }

  auto material1 = std::make_shared<material::Dielectric>(1.5);
  final_world.add(
      std::make_shared<objects::Sphere>(vec::Point3(0, 1, 0), 1.0, material1));

  auto material2 =
      std::make_shared<material::Lambertian>(color::Color(0.4, 0.2, 0.1));
  final_world.add(
      std::make_shared<objects::Sphere>(vec::Point3(-4, 1, 0), 1.0, material2));

  auto material3 =
      std::make_shared<material::Metal>(color::Color(0.7, 0.6, 0.5), 0.0);
  final_world.add(
      std::make_shared<objects::Sphere>(vec::Point3(4, 1, 0), 1.0, material3));

  // Camera
  camera::Camera cam;
  cam.image_width = 400;
  cam.samples_per_pixel = 20;
  cam.max_depth = 20;

  cam.fov = 20;
  cam.look_from = vec::Point3(13, 2, 3);
  cam.look_at = vec::Point3(0, 0, 0);
  cam.v_up = vec::Vec3(0, 1, 0);

  cam.focus_angle = 0.6;
  cam.focus_distance = 10.0;

  cam.render(final_world);

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
