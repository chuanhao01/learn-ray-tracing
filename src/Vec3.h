#ifndef VEC3_H
#define VEC3_H

#include <iostream>

namespace vec {
class Vec3 {
public:
  Vec3() : e{0, 0, 0} {}
  Vec3(double x, double y, double z) : e{x, y, z} {}

  static Vec3 random();
  static Vec3 random(double min, double max);
  static Vec3 random_in_unit_sphere();

  double x() { return e[0]; }
  double y() { return e[1]; }
  double z() { return e[2]; }

  double x() const { return e[0]; }
  double y() const { return e[1]; }
  double z() const { return e[2]; }

  // Overloading operators
  // Subscript access to values
  double operator[](int i) const { return e[i]; }
  double &operator[](int i) { return e[i]; }

  Vec3 operator-() const { return Vec3(-e[0], -e[1], -e[2]); }

  Vec3 &operator+=(const Vec3 &v);
  Vec3 &operator-=(const Vec3 &v);
  Vec3 &operator*=(double t);
  Vec3 &operator*=(const Vec3 &v);
  Vec3 &operator/=(double t);

  double length() const;
  double length_squared() const;

  bool near_zero() const;

private:
  double e[3];
};

std::ostream &operator<<(std::ostream &cout, const Vec3 &v);

Vec3 operator+(const Vec3 &v1, const Vec3 &v2);
Vec3 operator-(const Vec3 &v1, const Vec3 &v2);
Vec3 operator*(const Vec3 &v, double t);
Vec3 operator*(double t, const Vec3 &v);
Vec3 operator*(const Vec3 &v1, const Vec3 &v2);
Vec3 operator/(Vec3 &v, double t);

double dot(const Vec3 &v1, const Vec3 &v2);
Vec3 cross(const Vec3 &v1, const Vec3 &v2);

Vec3 unit_vector(const Vec3 &v);

Vec3 reflect(const Vec3 &v, const Vec3 &unit_normal);
Vec3 refract(const Vec3 &unit_vector, const Vec3 &unit_normal,
             double eta_over_eta_prime);

Vec3 random_unit_vector_in_unit_sphere();
Vec3 random_unit_vector_on_hemisphere(const Vec3 &normal);

using Point3 = Vec3; // 3D Point

} // namespace vec

#endif
