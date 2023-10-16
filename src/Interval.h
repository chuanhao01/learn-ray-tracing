#ifndef INTERVAL_H
#define INTERVAL_H

#include "Constants.h"

namespace interval {
class Interval {
public:
  double min, max;
  Interval() : min(+infinity), max(-infinity) {}
  Interval(double _min, double _max) : min(_min), max(_max) {}

  bool contains(double x) const { return min <= x && x <= max; }
  bool surrounds(double x) const { return min < x && x < max; }
  double clamp(double x) const;

  static const Interval empty, universe;
};

const static Interval empty();
const static Interval universe(-infinity, +infinity);
} // namespace interval

#endif
