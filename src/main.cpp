#include <iostream>

int main() {
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
    for (int i = 0; i < IMAGE_WIDTH; i++) {
      // Per row
      // Color mapping to 0.0 - 1.0 from 0 - 255
      // Need to have length wrap around 0 - 255
      auto r = double(i % COLOR_MAX) / COLOR_MAX;
      auto g = double(j % COLOR_MAX) / COLOR_MAX;
      auto b = 0.25;

      int ir = static_cast<int>(255.999 * r);
      int ig = static_cast<int>(255.999 * g);
      int ib = static_cast<int>(255.999 * b);

      std::cout << ir << " " << ig << " " << ib << "\n";
    }
  }
}
