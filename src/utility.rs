#[cfg(test)]
#[macro_use]
macro_rules! assert_eqf {
    ($e1:expr, $e2: expr, $tol: expr) => {
        assert!(($e1 - $e2).abs() <= $tol)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_case() {
        assert_eqf!(1.00001f64, 1.000002f64, 1e-4);
    }
}
