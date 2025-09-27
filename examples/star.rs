use std::f64::consts::PI;
use radiation_sim::physics::radiative_power;


fn main() {
    let r_m = 6.96e8_f64; // promień Słońca [m] (przykładowa wartość)
    let t_k = 5772.0_f64; // temp. efektywna [K]
    let area = 4.0 * PI * r_m * r_m;
    let eps = 1.0; // ciało doskonale czarne


    let p = radiative_power(t_k, area, eps);
    println!("Luminosity ≈ {:.3e} W", p);
}