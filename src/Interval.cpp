#include "Interval.h"

namespace interval {

double Interval::clamp(double x) const {
  if (x < min)
    return min;
  if (x > max)
    return max;
  return x;
}
} // namespace interval
