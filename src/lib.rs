/// Macro to test that two values are approximately equal. It checks that the relative difference between
/// the two values is less than some epsilon value.
///
/// ```ignore
/// use approx_eq::assert_approx_eq;
/// fn main() {
///   assert_approx_eq!(1., 0.99999999999); // should pass
///   assert_approx_eq!(1., 0.99999999999, 1e-5); // should pass
///   assert_approx_eq!(1., 0.99999999999, 1e-20); // should fail
///   assert_approx_eq!(0., 0.); // should pass
///   assert_approx_eq!(1., 2.); // should fail
/// }
/// ```

#[macro_use]
pub mod approx_eq {
    #[macro_export]
    /// Epsilon takes a default value of 1e-6.
    /// It is also possible to specify the error level to use.
    macro_rules! assert_approx_eq {
        ($x: expr, $y: expr) => {
            let eps = 1e-6;
            let (x, y): (f64, f64) = ($x, $y);
            if x == 0. {
                assert!(y.abs() < eps, "x = {}, y = {}", x, y);
            } else if y == 0. {
                assert!(x.abs() < eps, "x = {}, y = {}", x, y);
            } else {
                assert!(&x.signum() == &y.signum(), "x = {}, y = {}", x, y);
                let (x, y): (f64, f64) = (x.abs(), y.abs());
                assert!(
                    (&x - &y).abs() / [x, y].iter().cloned().fold(f64::NAN, f64::min) < eps,
                    "x = {}, y = {}",
                    x,
                    y
                );
            }
        };
        ($x: expr, $y: expr, $e: expr) => {
            let (x, y): (f64, f64) = ($x, $y);
            if x == 0. {
                assert!(y.abs() < $e, "x = {}, y = {}", x, y);
            } else if y == 0. {
                assert!(x.abs() < $e, "x = {}, y = {}", x, y);
            } else {
                assert!(&x.signum() == &y.signum(), "x = {}, y = {}", x, y);
                let (x, y): (f64, f64) = (x.abs(), y.abs());
                assert!(
                    (&x - &y).abs() / [x, y].iter().cloned().fold(f64::NAN, f64::min) < $e,
                    "x = {}, y = {}",
                    x,
                    y
                );
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noeps() {
        assert_approx_eq!(1., 1.);
        assert_approx_eq!(1., 1.000001);
    }

    #[test]
    fn test_witheps() {
        assert_approx_eq!(1.0000000001, 1., 1e-5);
    }

    #[test]
    fn test_nearzero() {
        assert_approx_eq!(0., -0.0000000000245);
    }

    #[test]
    fn test_zero() {
        assert_approx_eq!(0., 0.);
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        assert_approx_eq!(1.0000000001, 1., 1e-10);
    }

    #[test]
    #[should_panic]
    fn test_sign() {
        assert_approx_eq!(1., -1.);
    }
}
