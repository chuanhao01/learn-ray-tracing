#ifndef VEC3_H
#define VEC3_H

#include <cmath>
#include <iostream>

namespace vec {
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

  // Return a reference since this changes the left hand side to be the same
  Vec3 &operator+=(const Vec3 &v){};
  Vec3 &operator-=(const Vec3 &v){};
  Vec3 &operator*=(double t){};
  Vec3 &operator/=(double t){};

  double length() const {};

private:
  double e[3];
};

std::ostream &operator<<(std::ostream &cout, const Vec3 &v){};

Vec3 operator+(const Vec3 &v1, const Vec3 &v2){};
Vec3 operator-(const Vec3 &v1, const Vec3 &v2){};
Vec3 operator*(const Vec3 &v, double t){};
Vec3 operator*(double t, const Vec3 &v){};
Vec3 operator/(Vec3 &v, double t){};
Vec3 operator/(double t, Vec3 &v){};

double dot(const Vec3 &v1, const Vec3 &v2){};
Vec3 cross(const Vec3 &v1, const Vec3 &v2){};

Vec3 unit_vector(const Vec3 &v){};

using Point3 = Vec3; // 3D Point
using Color = Vec3;  //

} // namespace vec

#endif
