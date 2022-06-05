#ifndef VEC3_H
#define VEC3_H

#include <cmath>
#include <iostream>

class Vec3 {
public:
  Vec3() : e{0, 0, 0} {}
  Vec3(double x, double y, double z) : e{x, y, z} {}

  double x() { return e[0]; }
  double y() { return e[1]; }
  double z() { return e[2]; }

  // Return a new copy of the Vec3 for unary operator
  // So it can be used for chaining/assignment
  Vec3 operator-() const {};

  // Subscript
  double operator[](int i) const {};
  double &operator[](int i){};

  Vec3 &operator+=(const Vec3 &v){};
  Vec3 &operator-=(const Vec3 &v){};

private:
  double e[3];
};

#endif
