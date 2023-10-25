use std::f64::consts::PI;

// Converts a degree value into radians which we use internally
pub fn from_fdegree_to_fradian(degree: f64) -> f64 {
    degree * PI / 180_f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_degree() {
        assert_eq!(from_fdegree_to_fradian(90_f64), PI / 2_f64);
        assert_eq!(from_fdegree_to_fradian(180_f64), PI);
        assert_eq!(from_fdegree_to_fradian(270_f64), PI * (3_f64 / 2_f64));
        assert_eq!(from_fdegree_to_fradian(360_f64), PI * 2_f64);
    }
}
