#include "Color.h"
#include "Vec3.h"

#include <iostream>

int engine() {
  // Image
  const int IMAGE_WIDTH = 256;
  const int IMAGE_HEIGHT = 256;
  const int COLOR_MAX = 255;

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
      // Per row
      // Color mapping to 0.0 - 1.0 from 0 - 255
      // Need to have length wrap around 0 - 255
      auto r = double(i) / COLOR_MAX;
      auto g = double(j) / COLOR_MAX;
      auto b = 0;

      color::Color pixel_color(r, g, b);
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
