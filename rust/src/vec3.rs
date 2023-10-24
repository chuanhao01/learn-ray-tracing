use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
};

// Holds a 3D vector in the form of (x, y, z)
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }
    pub fn new_int(x: i64, y: i64, z: i64) -> Self {
        Vec3::new(x as f64, y as f64, z as f64)
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
}

// Operator overload for Vec3

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3: ({}, {}, {})", self[0], self[1], self[2])
    }
}

// impl AddAssign for Vec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//         self.z += rhs.z;
//     }
// }
// impl Add for Vec3 {
//     type Output = Vec3;
//     fn add(self, rhs: Self) -> Self::Output {
//         let mut v = self;
//         v += rhs;
//         v
//     }
// }

// impl SubAssign for Vec3 {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.x -= rhs.x;
//         self.y -= rhs.y;
//         self.z -= rhs.z;
//     }
// }
// impl Sub for Vec3 {
//     type Output = Vec3;
//     fn sub(self, rhs: Self) -> Self::Output {
//         let mut v = self;
//         v -= rhs;
//         v
//     }
// }

// // Hadamard product for Vec
// impl MulAssign for Vec3 {
//     fn mul_assign(&mut self, rhs: Self) {
//         self.x *= rhs.x;
//         self.y *= rhs.y;
//         self.z *= rhs.z;
//     }
// }
// impl Mul for Vec3 {
//     type Output = Vec3;
//     fn mul(self, rhs: Self) -> Self::Output {
//         let mut v = self;
//         v *= rhs;
//         v
//     }
// }

// impl MulAssign<f64> for Vec3 {
//     fn mul_assign(&mut self, rhs: f64) {
//         self.x *= rhs;
//         self.y *= rhs;
//         self.z *= rhs;
//     }
// }
// impl Mul<f64> for Vec3 {
//     type Output = Vec3;
//     fn mul(self, rhs: f64) -> Self::Output {
//         let mut v = self;
//         v *= rhs;
//         v
//     }
// }

// // Implement for left side f64 * Vec3
// impl Mul<Vec3> for f64 {
//     type Output = Vec3;
//     fn mul(self, rhs: Vec3) -> Self::Output {
//         rhs * self
//     }
// }

// impl DivAssign<f64> for Vec3 {
//     fn div_assign(&mut self, rhs: f64) {
//         *self *= 1.0 / rhs;
//     }
// }
// impl Div<f64> for Vec3 {
//     type Output = Vec3;
//     fn div(self, rhs: f64) -> Self::Output {
//         let mut v = self;
//         v /= rhs;
//         v
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init_vec3() {
        let v = Vec3 { e: [1.0, 1.0, 1.0] };
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 1.0);
        assert_eq!(v.z(), 1.0);

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
    fn test_vec3_neg() {
        let v1 = Vec3::new_int(1, 2, 4);
        let v1 = -v1;
        assert_eq!(v1.x(), -1.0);
        assert_eq!(v1.y(), -2.0);
        assert_eq!(v1.z(), -4.0);
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        v1 += v2;
        assert_eq!(v1.x, 3.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 5.0);
    }
    #[test]
    fn test_vec3_add() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        let v = v1 + v2;
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.z, 5.0);
    }

    #[test]
    fn test_vec3_sub_assign() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        v1 -= v2;
        assert_eq!(v1.x, -1.0);
        assert_eq!(v1.y, 0.0);
        assert_eq!(v1.z, 1.0);
    }
    #[test]
    fn test_vec3_sub() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        let v = v1 - v2;
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        v1 *= v2;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
    }
    #[test]
    fn test_vec3_mul() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v2 = Vec3::new_int(2, 2, 2);
        let r = v1 * v2;
        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, 4.0);
        assert_eq!(r.z, 6.0);
    }

    #[test]
    fn test_vec3_mul_assign_f64() {
        let mut v1 = Vec3::new_int(1, 2, 3);
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
    }
    #[test]
    fn test_vec3_mul_f64() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v = v1 * 2.0;
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.z, 6.0);
    }

    #[test]
    fn test_vec3_mul_f64_vec3() {
        let v1 = Vec3::new_int(1, 2, 3);
        let v = 2.0 * v1;
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.z, 6.0);
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut v1 = Vec3::new_int(1, 2, 4);
        v1 /= 2.0;
        assert_eq!(v1.x, 0.5);
        assert_eq!(v1.y, 1.0);
        assert_eq!(v1.z, 2.0);
    }
    #[test]
    fn test_vec3_div() {
        let v1 = Vec3::new_int(1, 2, 4);
        let v = v1 / 2.0;
        assert_eq!(v.x, 0.5);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 2.0);
    }
}
