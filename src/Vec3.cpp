#include "Vec3.h"
#include "RTWeekend.h"

#include <cmath>
#include <iostream>

namespace vec {

Vec3 Vec3::random() {
  return Vec3(random_double(), random_double(), random_double());
}
Vec3 Vec3::random(double min, double max) {
  return Vec3(random_double(min, max), random_double(min, max),
              random_double(min, max));
}

Vec3 Vec3::random_in_unit_sphere() {
  while (true) {
    auto v = Vec3::random(-1, 1);
    if (v.length_squared() < 1) {
      return v;
    }
  }
}

Vec3 &Vec3::operator+=(const Vec3 &v) {
  e[0] += v[0];
  e[1] += v[1];
  e[2] += v[2];
  return *this;
}
Vec3 &Vec3::operator-=(const Vec3 &v) {
  e[0] -= v[0];
  e[1] -= v[1];
  e[2] -= v[2];
  return *this;
}
Vec3 &Vec3::operator*=(double t) {
  e[0] *= t;
  e[1] *= t;
  e[2] *= t;
  return *this;
}
Vec3 &Vec3::operator*=(const Vec3 &v) {
  e[0] *= v[0];
  e[1] *= v[1];
  e[2] *= v[2];
  return *this;
}
Vec3 &Vec3::operator/=(double t) {
  *this *= 1 / t;
  return *this;
}

double Vec3::length() const { return std::sqrt(length_squared()); }
double Vec3::length_squared() const {
  return e[0] * e[0] + e[1] * e[1] + e[2] * e[2];
}

bool Vec3::near_zero() const {
  auto s = 1e-8;
  return (std::fabs(e[0]) < s) && (std::fabs(e[1]) < s) &&
         (std::fabs(e[2]) < s);
}

std::ostream &operator<<(std::ostream &cout, const Vec3 &v) {
  return cout << "Vec3(" << v[0] << ", " << v[1] << ", " << v[2] << ")";
}

Vec3 operator+(const Vec3 &v1, const Vec3 &v2) {
  Vec3 v = Vec3(v1);
  v += v2;
  return v;
}
Vec3 operator-(const Vec3 &v1, const Vec3 &v2) {
  Vec3 v = Vec3(v1);
  v -= v2;
  return v;
}
Vec3 operator*(const Vec3 &v, double t) {
  Vec3 nv = Vec3(v);
  nv *= t;
  return nv;
}
Vec3 operator*(double t, const Vec3 &v) {
  Vec3 nv = Vec3(v);
  nv *= t;
  return nv;
}
Vec3 operator*(const Vec3 &v1, const Vec3 &v2) {
  Vec3 nv = Vec3(v1);
  nv *= v2;
  return nv;
}
Vec3 operator/(Vec3 &v, double t) {
  Vec3 nv = Vec3(v);
  nv /= t;
  return nv;
}

// Since we are only dealing with 3D vecs, this is [3x1] * [1x3] always
double dot(const Vec3 &v1, const Vec3 &v2) {
  return v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
}
Vec3 cross(const Vec3 &v1, const Vec3 &v2) {
  return Vec3(v1[1] * v2[2] - v1[2] * v2[1], v1[2] * v2[0] - v1[0] * v2[2],
              v1[0] * v2[1] - v1[1] * v2[0]);
}

Vec3 reflect(const Vec3 &v, const Vec3 &unit_normal) {
  return v - 2 * dot(v, unit_normal) * unit_normal;
}
Vec3 refract(const Vec3 &unit_vector, const Vec3 &unit_normal,
             double eta_over_eta_prime) {
  auto cos_theta = std::fmin(dot(-unit_vector, unit_normal), 1.0);
  Vec3 r_out_perpendicular =
      eta_over_eta_prime * (unit_vector + cos_theta * unit_normal);
  Vec3 r_out_parrallel =
      -std::sqrt(std::fabs(1.0 - r_out_perpendicular.length_squared())) *
      unit_normal;
  return r_out_perpendicular + r_out_parrallel;
}

Vec3 unit_vector(const Vec3 &v) {
  Vec3 nv = Vec3(v);
  return nv / nv.length();
}

Vec3 random_unit_vector_in_unit_sphere() {
  return unit_vector(Vec3::random_in_unit_sphere());
}

Vec3 random_unit_vector_on_hemisphere(const Vec3 &unit_normal) {
  Vec3 on_unit_sphere = random_unit_vector_in_unit_sphere();
  if (dot(on_unit_sphere, unit_normal) > 0.0) {
    // Same side
    return on_unit_sphere;
  }
  return -on_unit_sphere;
}
} // namespace vec
