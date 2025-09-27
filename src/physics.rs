/// Stała Stefana–Boltzmanna [W·m^-2·K^-4]
pub const SIGMA: f64 = 5.670_374_419e-8;


/// Moc promieniowania: P = σ · ε · A · T^4
pub fn radiative_power(temp_k: f64, area_m2: f64, emissivity: f64) -> f64 {
    SIGMA * emissivity * area_m2 * temp_k.powi(4)
}


/// Energia wypromieniowana w czasie: E = P · t
pub fn radiated_energy(power_w: f64, time_s: f64) -> f64 {
    power_w * time_s
}