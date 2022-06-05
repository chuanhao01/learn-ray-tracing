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
    std::cerr << "\r"
              << "Scanlines remaining: " << IMAGE_HEIGHT - j - 1 << std::flush;
    for (int i = 0; i < IMAGE_WIDTH; i++) {
      // Per row
      // Color mapping to 0.0 - 1.0 from 0 - 255
      // Need to have length wrap around 0 - 255
      auto r = double(i % (COLOR_MAX + 1)) / COLOR_MAX;
      auto g = double(j % (COLOR_MAX + 1)) / COLOR_MAX;
      auto b = 0.25;

      // Math is because truncation (Prob)
      // Did some test, IDK, Math
      // So 0 - 255 = 0 - 254.9999 = 0 - 254
      // Hence needs 0 - 256 = 0 - 255.999 = 0 - 255
      int ir = static_cast<int>(255.999 * r);
      int ig = static_cast<int>(255.999 * g);
      int ib = static_cast<int>(255.999 * b);

      std::cout << ir << " " << ig << " " << ib << "\n";
    }
  }
  std::cerr << "\n"
            << "Done!"
            << "\n";
}
