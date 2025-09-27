use anyhow::{anyhow, Result};


use crate::cli::Output;
use crate::cli::Cli;


/// Parametry po walidacji.
#[derive(Debug, Clone)]
pub struct Params {
    pub temp: f64,
    pub area: f64,
    pub time: f64,
    pub emissivity: f64,
    pub precision: usize,
    pub output: Output,
}


pub fn validate_params(cli: Cli) -> Result<Params> {
    if !cli.temp.is_finite() || cli.temp <= 0.0 {
        return Err(anyhow!("Temperatura musi być > 0 K"));
    }
    if !cli.area.is_finite() || cli.area < 0.0 {
        return Err(anyhow!("Powierzchnia musi być ≥ 0 m²"));
    }
    if !cli.time.is_finite() || cli.time < 0.0 {
        return Err(anyhow!("Czas musi być ≥ 0 s"));
    }
    if !cli.emissivity.is_finite() || cli.emissivity < 0.0 || cli.emissivity > 1.0 {
        return Err(anyhow!("Emisyjność ε musi leżeć w przedziale [0, 1]"));
    }
    if cli.precision > 10 {
        return Err(anyhow!("Precision musi być w zakresie 0..=10"));
    }


    Ok(Params {
        temp: cli.temp,
        area: cli.area,
        time: cli.time,
        emissivity: cli.emissivity,
        precision: cli.precision as usize,
        output: cli.output,
    })
}