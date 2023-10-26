use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rand::{thread_rng, Rng};

// Holds a 3D vector in the form of (x, y, z)
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    /// Creates a new Vec3 with f64 (x, y, z)
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }
    /// Creates a new Vec3 with i64 (x, y, z)
    pub fn new_int(x: i64, y: i64, z: i64) -> Self {
        Vec3::new(x as f64, y as f64, z as f64)
    }

    /// Getter for x
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    /// Getter for y
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    /// Getter for z
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// Returns the vector as a tuple for easy destructuring
    pub fn tuple(&self) -> (f64, f64, f64) {
        (self.e[0], self.e[1], self.e[2])
    }

    /// Intermediate calculation for length of vector squared
    /// Used in [Vec3::length]
    /// Useful when the expensive sqrt is not needed
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = -1e8_f64;
        self.e[0] < s && self.e[1] < s && self.e[2] < s
    }

    // Public fns
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }
    /// Generates a random vector with x, y and z in (min, max)
    pub fn random(min: f64, max:f64) -> Vec3{
        let mut rng = thread_rng();
        Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }
    /// Samples a random vector inside a unit sphere with, Center(0, 0, 0) radius=1
    pub fn random_vector_in_unit_sphere() -> Vec3{
        loop {
            let v = Vec3::random(-1_f64, 1_f64);
            if v.length_squared() < 1_f64{
                break v
            }
        }
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 { e: self.e }
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3: ({}, {}, {})", self[0], self[1], self[2])
    }
}
/// Implemented for ease of testing
impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Vec3 as Display>::fmt(self, f)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

// Operator overload for Vec3
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self;
        v += rhs;
        v
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = self;
        v -= rhs;
        v
    }
}

// // Hadamard product for Vec
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut v = self;
        v *= rhs;
        v
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut v = self;
        v *= rhs;
        v
    }
}

// Implement for left side f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        let mut v = self;
        v /= rhs;
        v
    }
}

/// Implemented for ease of testing
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e[0] == other.e[0] && self.e[1] == other.e[1] && self.e[2] == other.e[2]
    }
}
impl Eq for Vec3 {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init_vec3() {
        let v = Vec3 { e: [1.0, 1.0, 1.0] };
        assert_eq!(v.e[0], 1.0);
        assert_eq!(v.e[1], 1.0);
        assert_eq!(v.e[2], 1.0);
    }

    #[test]
    fn test_vec3_new() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 1.0);
        assert_eq!(v.z(), 1.0);
    }
    #[test]
    fn test_vec3_new_int() {
        let v = Vec3::new_int(1, 1, 1);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 1.0);
        assert_eq!(v.z(), 1.0);
    }

    #[test]
    fn test_vec3_length_squared() {
        let v = Vec3::new_int(1, 1, 1);
        assert_eq!(v.length_squared(), 3.0);
    }
    #[test]
    fn test_vec3_length() {
        let v = Vec3::new_int(1, 1, 1);
        assert_eq!(v.length(), 3.0_f64.sqrt());
    }

    #[test]
    fn test_vec3_unit() {
        // Simple Unit Vector
        let v = Vec3::new_int(3, 0, 0);
        let u_v = v.unit_vector();
        assert_eq!(u_v, Vec3::new_int(1, 0, 0));

        let v = Vec3::new_int(3, 3, 3);
        let u_v = v.unit_vector();
        let u_l = 3_f64 / (9_f64 * 3_f64).sqrt();
        assert_eq!(u_v, Vec3::new(u_l, u_l, u_l));
    }

    #[test]
    fn test_vec3_clone() {
        let v = Vec3::new_int(1, 1, 1);
        let v1 = v.clone();
        assert_eq!(v, v1);
    }

    // Vec3 Operator Overload Tests
    #[test]
    fn test_vec3_neg() {
        let v1 = Vec3::new_int(1, 2, 4);
        let v1 = -v1;
        assert_eq!(v1, Vec3::new(-1.0, -2.0, -4.0));
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        v1 += v2;
        assert_eq!(v1, Vec3::new_int(3, 4, 5));
    }
    #[test]
    fn test_vec3_add() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        let v = v1 + v2;
        assert_eq!(v, Vec3::new_int(3, 4, 5));
    }

    #[test]
    fn test_vec3_sub_assign() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        v1 -= v2;
        assert_eq!(v1, Vec3::new_int(-1, 0, 1));
    }
    #[test]
    fn test_vec3_sub() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        let v = v1 - v2;
        assert_eq!(v, Vec3::new_int(-1, 0, 1));
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        v1 *= v2;
        assert_eq!(v1, Vec3::new_int(2, 4, 6));
    }
    #[test]
    fn test_vec3_mul() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        let r = v1 * v2;
        assert_eq!(r, Vec3::new_int(2, 4, 6));
    }

    #[test]
    fn test_vec3_mul_assign_f64() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        v1 *= 2.0;
        assert_eq!(v1, Vec3::new_int(2, 4, 6));
    }
    #[test]
    fn test_vec3_mul_f64() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v = v1 * 2.0;
        assert_eq!(v, Vec3::new_int(2, 4, 6));
    }

    #[test]
    fn test_vec3_mul_f64_vec3() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v = 2.0 * v1;
        assert_eq!(v, Vec3::new_int(2, 4, 6));
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut v1 = Vec3::new_int(1, 2, 4);
        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(0.5, 1.0, 2.0));
    }
    #[test]
    fn test_vec3_div() {
        let v1 = Vec3::new_int(1, 2, 4);
        let v = v1 / 2.0;
        assert_eq!(v, Vec3::new(0.5, 1.0, 2.0));
    }

    // Public Functions Tests
    #[test]
    fn test_dot() {
        let u = Vec3::new_int(1, 1, 1);
        let v = Vec3::new_int(2, 2, 2);
        assert_eq!(Vec3::dot(&u, &v), 6.0);
    }
    #[test]
    fn test_cross() {
        let u = Vec3::new_int(1, 1, 1);
        let v = Vec3::new_int(-2, 2, 2);
        let r = Vec3::cross(&u, &v);
        assert_eq!(r, Vec3::new_int(0, -4, 4));
    }
}
