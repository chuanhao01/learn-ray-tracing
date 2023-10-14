#include "RTWeekend.h"

double degrees_to_radians(double degrees) { return degrees / 180.0 * PI; }

/**
 * @brief
 * Returns a random from [0, 1)
 * @return
 */
double random_double() { return std::rand() / (RAND_MAX + 1); }
double random(double min, double max) {
  return min + (max - min) * random_double();
}
