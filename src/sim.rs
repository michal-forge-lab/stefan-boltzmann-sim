use anyhow::Result;
use serde::Serialize;


use crate::physics::{radiated_energy, radiative_power};
use crate::validate::Params;


/// Wynik symulacji.
#[derive(Debug, Clone, Serialize)]
pub struct SimulationResult {
    pub power_w: f64,
    pub energy_j: f64,
}


/// Wykonuje pojedynczy krok symulacji dla stałej temperatury
/// (brak modelu stygnięcia – moc stała w czasie).
pub fn run(params: &Params) -> Result<SimulationResult> {
    let p = radiative_power(params.temp, params.area, params.emissivity);
    let e = radiated_energy(p, params.time);
    Ok(SimulationResult {
        power_w: p,
        energy_j: e,
    })
}