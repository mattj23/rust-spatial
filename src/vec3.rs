#[warn(missing_debug_implementations, missing_debug_implementations)]
use std::ops;

/// Struct for holding a standard vector in 3-space
#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Create a zero-valued Vec3 which represents a zero length vector
    pub fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    /// Create a new vector by specifying the x, y, and z components
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // pub fn dot(self, )
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;


    #[test]
    fn vector_add() {
        let v0 = Vec3::new(3., 4., 5.);
        let v1 = Vec3::new(10., 12., 14.);
        let a = v0 + v1;
        assert_eqf!(a.x, 13., 1e-10);
        assert_eqf!(a.y, 16., 1e-10);
        assert_eqf!(a.z, 19., 1e-10);
    }

    #[test]
    fn vector_empty() {
        let v = Vec3::zero();
        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
        assert_eq!(v.z, 0.);
    }

    #[test]
    fn vector_new() {
        let v = Vec3::new(3., 4., 5.);
        assert_eq!(v.x, 3.);
        assert_eq!(v.y, 4.);
        assert_eq!(v.z, 5.);
    }
}
