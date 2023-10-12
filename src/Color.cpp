#include "Color.h"

#include <iostream>

namespace color {
/**
 * Take a color pixel and prints out the rgb values to cout
 */
void write_color(std::ostream &os, const Color &pixel_color) {
  os << static_cast<int>(pixel_color[0] * 255.999) << " "
     << static_cast<int>(pixel_color[1] * 255.999) << " "
     << static_cast<int>(pixel_color[2] * 255.999);
}
} // namespace color
