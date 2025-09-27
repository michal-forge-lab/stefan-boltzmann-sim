use approx::assert_relative_eq;


// Dostęp do modułów biblioteki (src/lib.rs)
use radiation_sim::physics::{radiative_power, SIGMA};


#[test]
fn power_matches_sigma_t4() {
    let t = 500.0_f64;
    let a = 2.0_f64;
    let eps = 1.0_f64;


    let p = radiative_power(t, a, eps);
    let expected = SIGMA * eps * a * t.powi(4);


    assert_relative_eq!(p, expected, max_relative = 1e-12);
}