//! Macro to test that two values are approximately equal. It checks that the relative difference between
//! the two values is less than some epsilon value.

#[macro_use]
pub mod approx_eq {
    #[macro_export]
    /// Epsilon takes a default value of 1e-6.
    /// It is also possible to specify the error level to use.
    macro_rules! assert_approx_eq {
        ($x: expr, $y: expr) => {
            let eps = 1e-6;
            let (x, y): (f64, f64) = ($x, $y);
            assert!(&x.signum() == &y.signum());
            assert!((&x - &y).abs() / [x, y].iter().cloned().fold(0. / 0., f64::min) < eps);
        };
        ($x: expr, $y: expr, $e: expr) => {
            let (x, y): (f64, f64) = ($x, $y);
            assert!(&x.signum() == &y.signum());
            assert!((&x - &y).abs() / [x, y].iter().cloned().fold(0. / 0., f64::min) < $e)
        };
    }
}

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
#[should_panic(expected = "assertion failed")]
fn test_invalid() {
    assert_approx_eq!(1.0000000001, 1., 1e-10);
}

#[test]
fn test_sign() {
    assert_approx_eq!(1., -1.);
}
