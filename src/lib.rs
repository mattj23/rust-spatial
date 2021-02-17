#[warn(missing_debug_implementations, missing_debug_implementations)]

/// Struct for holding a standard vector in 3-space
#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn vector_empty() {
        let v = Vec3::new();
        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
        assert_eq!(v.z, 0.);
    }
}
