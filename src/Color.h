#ifndef COLOR_H
#define COLOR_H

#include "Vec3.h"

#include <iostream>

namespace color {
using Color = vec::Vec3;

void write_color(std::ostream &os, const Color &pixel_color,
                 int samples_per_pixel);
} // namespace color

#endif
