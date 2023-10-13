#ifndef VEC3_H
#define VEC3_H

#include <iostream>

namespace vec {
class Vec3 {
public:
  Vec3() : e{0, 0, 0} {}
  Vec3(double x, double y, double z) : e{x, y, z} {}

  double x() { return e[0]; }
  double y() { return e[1]; }
  double z() { return e[2]; }

  // Overloading operators
  // Subscript access to values
  double operator[](int i) const { return e[i]; }
  double &operator[](int i) { return e[i]; }

  Vec3 operator-() const { return Vec3(-e[0], -e[1], -e[2]); }

  Vec3 &operator+=(const Vec3 &v);
  Vec3 &operator-=(const Vec3 &v);
  Vec3 &operator*=(double t);
  Vec3 &operator/=(double t);

  double length() const;
  double length_squared() const;

private:
  double e[3];
};

std::ostream &operator<<(std::ostream &cout, const Vec3 &v);

Vec3 operator+(const Vec3 &v1, const Vec3 &v2);
Vec3 operator-(const Vec3 &v1, const Vec3 &v2);
Vec3 operator*(const Vec3 &v, double t);
Vec3 operator*(double t, const Vec3 &v);
Vec3 operator/(Vec3 &v, double t);

double dot(const Vec3 &v1, const Vec3 &v2);
Vec3 cross(const Vec3 &v1, const Vec3 &v2);

Vec3 unit_vector(const Vec3 &v);

using Point3 = Vec3; // 3D Point

} // namespace vec

#endif
